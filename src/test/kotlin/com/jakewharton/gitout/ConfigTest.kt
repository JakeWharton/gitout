package com.jakewharton.gitout

import assertk.assertThat
import assertk.assertions.isEqualTo
import org.junit.Test

class ConfigTest {
	@Test fun empty() {
		val config = """
			|version = 0
			""".trimMargin()
		val expected = Config(version = 0)
		assertThat(Config.parse(config)).isEqualTo(expected)
	}

	@Test fun full() {
		val config = """
			|version = 0
			|
			|[github]
			|user = "user"
			|token = "token"
			|
			|[github.archive]
			|owned = false
			|#repos = [
			|#	"example/one"
			|#]
			|
			|[github.clone]
			|starred = true
			|watched = true
			|gists = false
			|repos = [
			|  "example/two",
			|]
			|ignore = [
			|  "hey/there",
			|]
			|
			|[git.repos]
			|example = "https://example.com/example.git"
			""".trimMargin()
		val expected = Config(
			version = 0,
			github = Config.GitHub(
				user = "user",
				token = "token",
				archive = Config.GitHub.Archive(
					owned = false,
				),
				clone = Config.GitHub.Clone(
					starred = true,
					watched = true,
					gists = false,
					repos = listOf(
						"example/two",
					),
					ignore = listOf(
						"hey/there",
					),
				),
			),
			git = Config.Git(
				repos = mapOf(
					"example" to "https://example.com/example.git",
				),
			),
		)
		assertThat(Config.parse(config)).isEqualTo(expected)
	}
}
