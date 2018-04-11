require "helix_runtime"

begin
  require "capybara-test/native"
rescue LoadError
  warn "Unable to load capybara-test/native. Please run `rake build`"
end
