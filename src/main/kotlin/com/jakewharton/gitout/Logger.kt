package com.jakewharton.gitout

internal class Logger(
	quiet: Boolean,
	private val level: Int,
) {
	init {
		require(level in 0..3) { "Level must be [0,3]: $level" }
	}

	val lifecycleEnabled = !quiet
	val infoEnabled = !quiet && level > 0
	val debugEnabled = !quiet && level > 1
	val traceEnabled = !quiet && level > 2

	fun warn(message: String) {
		println("WARN  $message")
	}

	inline fun lifecycle(message: () -> String) {
		if (lifecycleEnabled) {
			lifecycleInternal(message())
		}
	}

	@PublishedApi
	internal fun lifecycleInternal(message: String) {
		val s = if (level > 0) {
			"LOG   $message"
		} else {
			message
		}
		println(s)
	}

	inline fun info(message: () -> String) {
		if (infoEnabled) {
			println("INFO  ${message()}")
		}
	}

	inline fun debug(message: () -> String) {
		if (debugEnabled) {
			println("DEBUG ${message()}")
		}
	}

	inline fun trace(message: () -> String) {
		if (traceEnabled) {
			println("TRACE ${message()}")
		}
	}
}
