package com.jakewharton.gitout

import java.util.concurrent.TimeUnit.SECONDS
import kotlin.time.Duration

internal fun Process.waitFor(timeout: Duration): Boolean {
	return timeout.toComponents { seconds, nanoseconds ->
		var seconds = seconds
		if (nanoseconds != 0 && seconds < Long.MAX_VALUE) {
			seconds += 1
		}
		waitFor(seconds, SECONDS)
	}
}
