@file:JvmName("Main")

package com.jakewharton.gitout

import com.github.ajalt.clikt.command.SuspendingCliktCommand
import com.github.ajalt.clikt.command.main
import com.github.ajalt.clikt.parameters.arguments.argument
import com.github.ajalt.clikt.parameters.arguments.help
import com.github.ajalt.clikt.parameters.options.convert
import com.github.ajalt.clikt.parameters.options.counted
import com.github.ajalt.clikt.parameters.options.default
import com.github.ajalt.clikt.parameters.options.flag
import com.github.ajalt.clikt.parameters.options.help
import com.github.ajalt.clikt.parameters.options.option
import com.github.ajalt.clikt.parameters.options.versionOption
import com.github.ajalt.clikt.parameters.types.path
import io.github.kevincianfarini.cardiologist.PulseBackpressureStrategy.Companion.SkipNext
import io.github.kevincianfarini.cardiologist.PulseSchedule
import io.github.kevincianfarini.cardiologist.schedulePulse
import java.nio.file.FileSystem
import java.nio.file.FileSystems
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.Duration.Companion.minutes
import kotlinx.datetime.TimeZone
import okhttp3.HttpUrl.Companion.toHttpUrl
import okhttp3.OkHttpClient
import okhttp3.logging.HttpLoggingInterceptor
import okhttp3.logging.HttpLoggingInterceptor.Level.BODY

public suspend fun main(vararg args: String) {
	val systemFs = FileSystems.getDefault()!!
	val systemClock = Clock.System
	val systemTimeZone = TimeZone.currentSystemDefault()
	GitOutCommand(systemFs, systemClock, systemTimeZone).main(args)
}

private class GitOutCommand(
	fs: FileSystem,
	private val clock: Clock,
	private val timeZone: TimeZone,
) : SuspendingCliktCommand(name = "gitout") {
	init {
		versionOption(version)
	}

	private val config by argument()
		.path(mustExist = true, canBeDir = false, fileSystem = fs)
		.help("Configuration TOML")

	private val destination by argument()
		.path(mustExist = true, canBeFile = false, fileSystem = fs)
		.help("Backup directory")

	private val timeout by option(envvar = "GITOUT_TIMEOUT")
		.convert { Duration.parse(it) }
		.default(10.minutes)
		.help("Timeout for git clone/update operations (default: 10m)")

	private val verbosity by option("--verbose", "-v")
		.counted(limit = 3)
		.help("Increase logging verbosity. -v = informational, -vv = debug, -vvv = trace")

	private val quiet by option("--quiet", "-q")
		.flag()
		.help("Decrease logging verbosity. Takes precedence over verbosity")

	// TODO private val archive by option("--experimental-archive")
	//      	.flag()
	//      	.help("Enable EXPERIMENTAL repository archive")

	private val dryRun by option(envvar = "GITOUT_DRY_RUN")
		.flag()
		.help("Print actions instead of performing them")

	private val schedule by option("--cron", metavar = "expression", envvar = "GITOUT_CRON")
		.help("Run command forever and perform sync on this schedule")
		.convert { PulseSchedule.parseCron(it) }

	private val healthCheckId by option("--hc-id", metavar = "id", envvar = "GITOUT_HC_ID")
		.help("ID of Healthchecks.io service to notify")

	private val healthCheckHost by option("--hc-host", metavar = "url", envvar = "GITOUT_HC_HOST")
		.convert { it.toHttpUrl() }
		.default("https://hc-ping.com".toHttpUrl())
		.help("Host of Healthchecks.io service to notify. Requires --hc-id")

	override suspend fun run() {
		val logger = Logger(quiet, verbosity)

		val client = OkHttpClient.Builder()
			.addNetworkInterceptor(
				HttpLoggingInterceptor {
					logger.trace { it }
				}.also {
					it.level = BODY
				}
			)
			.build()

		val healthCheckService = HealthCheckService(healthCheckHost, client, logger)
		val healthCheck = healthCheckId?.let(healthCheckService::newCheck)

		val engine = Engine(
			config = config,
			destination = destination,
			timeout = timeout,
			logger = logger,
			client = client,
			healthCheck = healthCheck,
		)

		val schedule = schedule
		if (schedule != null) {
			logger.lifecycle { "Sync schedule: $schedule" }
			val pulse = clock.schedulePulse(schedule, timeZone)
			pulse.beat(strategy = SkipNext) {
				logger.lifecycle { "Schedule trigger for $it, executing at ${clock.now()}" }
				engine.performSync(dryRun)
			}
		} else {
			engine.performSync(dryRun)
		}

		client.connectionPool.evictAll()
		client.dispatcher.executorService.shutdown()
	}
}
