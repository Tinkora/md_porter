# frozen_string_literal: true

require "set"

required = %w[README.md README.zh-CN.md LICENSE CONTRIBUTING.md SECURITY.md SUPPORT.md CHANGELOG.md]
tracked = `git ls-files -z`.split("\0").reject(&:empty?).to_set
errors = []

required.each do |path|
  errors << "Missing required file: #{path}" unless File.file?(path)
end

utf8_bom = "\xEF\xBB\xBF".dup.force_encoding(Encoding::UTF_8)
(tracked.to_a + required).uniq.select { |path| path.end_with?(".md", ".yml", ".yaml", ".toml", ".json") }.sort.each do |path|
  content = File.binread(path).force_encoding(Encoding::UTF_8)
  errors << "UTF-8 BOM is not allowed: #{path}" if content.start_with?(utf8_bom)
  errors << "Invalid UTF-8: #{path}" unless content.valid_encoding?
end

errors.empty? ? puts("Documentation checks passed.") : errors.each { |error| warn(error) }
exit(errors.empty? ? 0 : 1)
