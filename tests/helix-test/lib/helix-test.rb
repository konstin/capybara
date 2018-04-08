require "helix_runtime"

begin
  require "helix-test/native"
rescue LoadError
  warn "Unable to load helix-test/native. Please run `rake build`"
end
