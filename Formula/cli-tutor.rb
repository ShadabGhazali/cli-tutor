class CliTutor < Formula
  desc "Interactive terminal app for learning Unix command-line tools"
  homepage "https://github.com/ShadabGhazali/cli-tutor"
  url "https://github.com/ShadabGhazali/cli-tutor/archive/refs/tags/v0.3.0.tar.gz"
  sha256 "cc308b31fa2ba3dae615894fd25c69051b4cbaf5b8776aa782155796d8dc1b32"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cli-tutor --version 2>&1", 1)
  end
end
