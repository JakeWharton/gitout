package com.jakewharton.gitout

import okhttp3.HttpUrl
import okhttp3.OkHttpClient
import okhttp3.Request

internal class HealthCheckService(
	private val host: HttpUrl,
	private val client: OkHttpClient,
	private val logger: Logger,
) {
	fun newCheck(id: String): HealthCheck {
		val url = host.newBuilder()
			.addPathSegment(id)
			.build()
		return HealthCheck(url, client, logger)
	}
}

internal class HealthCheck(
	private val url: HttpUrl,
	private val client: OkHttpClient,
	private val logger: Logger,
) {
	fun start(): Started {
		val startUrl = url
			.newBuilder()
			.addPathSegment("start")
			.build()

		logger.debug { "Healthcheck start $startUrl" }
		client.newCall(Request(url = startUrl, method = "POST")).execute()

		return Started(url, client, logger)
	}

	class Started(
		private val url: HttpUrl,
		private val client: OkHttpClient,
		private val logger: Logger,
	) {
		fun complete() {
			logger.debug { "Healthcheck complete $url" }
			client.newCall(Request(url = url, method = "POST")).execute()
		}
	}
}
