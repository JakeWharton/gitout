package com.jakewharton.gitout

import com.akuleshov7.ktoml.Toml
import dev.drewhamilton.poko.Poko
import kotlinx.serialization.Serializable

@Poko
@Serializable
internal class Config(
	val version: Int,
	val github: GitHub? = null,
	val git: Git = Git(),
) {
	companion object {
		private val format = Toml

		fun parse(toml: String): Config {
			return format.decodeFromString(serializer(), toml)
		}
	}

	@Poko
	@Serializable
	class GitHub(
		val user: String,
		val token: String,
		val archive: Archive = Archive(),
		val clone: Clone = Clone(),
	) {
		@Poko
		@Serializable
		class Archive(
			val owned: Boolean = false,
		)
		@Poko
		@Serializable
		class Clone(
			val starred: Boolean = false,
			val watched: Boolean = false,
			val gists: Boolean = true,
			val repos: List<String> = emptyList(),
			val ignore: List<String> = emptyList(),
		)
	}

	@Poko
	@Serializable
	class Git(
		val repos: Map<String, String> = emptyMap(),
	)
}
