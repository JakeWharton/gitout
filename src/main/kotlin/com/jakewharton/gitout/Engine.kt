package com.jakewharton.gitout

import dev.drewhamilton.poko.Poko
import java.nio.file.Path
import java.util.concurrent.TimeUnit.MINUTES
import kotlin.io.path.exists
import kotlin.io.path.isDirectory
import kotlin.io.path.name
import kotlin.io.path.notExists
import kotlin.io.path.readText
import okhttp3.HttpUrl
import okhttp3.OkHttpClient

internal class Engine(
	private val config: Path,
	private val destination: Path,
	private val logger: Logger,
	private val client: OkHttpClient,
	private val healthCheck: HealthCheck?,
) {
	suspend fun performSync(dryRun: Boolean) {
		val startedHealthCheck = healthCheck?.start()

		val config = Config.parse(config.readText())
		logger.trace { config.toString() }
		check(config.version == 0) {
			"Only version 0 of the config is supported at this time"
		}

		check(dryRun || destination.exists() && destination.isDirectory()) {
			"Destination must exist and must be a directory"
		}

		if (config.github != null) {
			val gitHub = GitHub(config.github.user, config.github.token, client, logger)
			val githubDestination = destination.resolve("github")

			logger.info { "Querying GitHub information for ${config.github.user}…" }
			val githubRepositories = gitHub.loadRepositories()
			logger.trace { githubRepositories.toString() }
			logger.info {
				"""
				|Repositories
				|  owned: ${githubRepositories.owned.size}
				|  starred: ${githubRepositories.starred.size}
				|  watching: ${githubRepositories.watching.size}
				|  gists: ${githubRepositories.gists.size}
				""".trimMargin()
			}

			val nameAndOwnerToReasons = mutableMapOf<String, MutableSet<String>>()

			for (nameAndOwner in githubRepositories.owned) {
				logger.debug { "Owned candidate $nameAndOwner" }
				nameAndOwnerToReasons.getOrPut(nameAndOwner, ::HashSet).add("owned")
			}

			for (nameAndOwner in config.github.clone.repos) {
				logger.debug { "Explicit candidate $nameAndOwner" }
				nameAndOwnerToReasons.getOrPut(nameAndOwner, ::HashSet).add("explicit")
			}

			if (config.github.clone.starred) {
				for (nameAndOwner in githubRepositories.starred) {
					logger.debug { "Starred candidate $nameAndOwner" }
					nameAndOwnerToReasons.getOrPut(nameAndOwner, ::HashSet).add("starred")
				}
			} else {
				logger.debug { "Starred disabled by config" }
			}

			if (config.github.clone.watched) {
				for (nameAndOwner in githubRepositories.watching) {
					logger.debug { "Watching candidate $nameAndOwner" }
					nameAndOwnerToReasons.getOrPut(nameAndOwner, ::HashSet).add("watching")
				}
			} else {
				logger.debug { "Watching disabled by config" }
			}

			for (ignore in config.github.clone.ignore) {
				val reasons = nameAndOwnerToReasons.remove(ignore)
				if (reasons != null) {
					logger.debug { "Ignoring $ignore, was $reasons" }
				} else {
					logger.warn("Unused ignore: $ignore")
				}
			}

			val credentials = Credentials(
				username = config.github.user,
				password = if (dryRun) "dryrun!" else config.github.token,
			)

			val cloneDestination = githubDestination.resolve("clone")
			for ((nameAndOwner, reasons) in nameAndOwnerToReasons) {
				val repoDestination = cloneDestination.resolve(nameAndOwner)
				val repoUrl = HttpUrl.Builder()
					.scheme("https")
					.host("github.com")
					.build()
					.resolve("$nameAndOwner.git")
					.toString()
				logger.lifecycle { "Synchronizing $repoUrl because $reasons…" }
				syncBare(repoDestination, repoUrl, dryRun, credentials)
			}

			if (config.github.clone.gists) {
				val gistsDestination = githubDestination.resolve("gists")
				for (gist in githubRepositories.gists) {
					val gistDestination = gistsDestination.resolve(gist)
					val gistUrl = HttpUrl.Builder()
						.scheme("https")
						.host("gist.github.com")
						.addPathSegment("$gist.git")
						.toString()
					logger.lifecycle { "Synchronizing $gistUrl…" }
					syncBare(gistDestination, gistUrl, dryRun, credentials)
				}
			} else {
				logger.debug { "Gists disabled by config" }
			}
		} else {
			logger.debug { "GitHub absent from config" }
		}

		val gitDestination = destination.resolve("git")
		for ((name, url) in config.git.repos) {
			val repoDestination = gitDestination.resolve(name)
			logger.lifecycle { "Synchronizing $name $url…" }
			syncBare(repoDestination, url, dryRun)
		}

		startedHealthCheck?.complete()
	}

	private fun syncBare(repo: Path, url: String, dryRun: Boolean, credentials: Credentials? = null) {
		val command = mutableListOf("git")
		if (credentials != null) {
			command.add("-c")
			command.add("""credential.helper='!f() { echo "username=${credentials.username}"; echo "password=${credentials.password}"; }; f'""")
		}

		val directory = if (repo.notExists()) {
			command.apply {
				add("clone")
				add("--mirror")
				add(url)
				add(repo.name)
			}
			repo.parent
		} else {
			command.apply {
				add("remote")
				add("update")
				add("--prune")
			}
			repo
		}

		if (dryRun) {
			logger.lifecycle { "DRY RUN $directory ${command.joinToString(separator = " ")}" }
		} else {
			val process = ProcessBuilder()
				.command(command)
				.directory(directory.toFile())
				.start()
			check(process.waitFor(5, MINUTES)) { "Unable to sync $url into $repo: 5m timeout" }
			check(process.exitValue() == 0) { "Unable to sync $url into $repo: exit ${process.exitValue()}" }
		}
	}

	@Poko
	private class Credentials(
		val username: String,
		val password: String,
	)
}
