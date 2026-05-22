# SPDX-License-Identifier: MPL-2.0
# config.ncl - BetLang Playground Configuration
# Type-safe configuration using Nickel
# See: https://nickel-lang.org

{
  # Project metadata
  project = {
    name = "betlang-playground",
    version = "1.0.0",
    description = "Playground for BetLang Playground",
    license = "MPL-2.0",
    author = "Jonathan D.A. Jewell",
    repository = "https://github.com/hyperpolymath/betlang-playground",
  },

  # Documentation configuration
  docs = {
    readme = "README.adoc",
  },

  # Development settings
  development = {
    default_tasks = [
      "build",
      "lint",
      "fmt",
      "test",
    ],
  },

  # Computed values
  full_name = project.name ++ " v" ++ project.version,
}
