class CliTutor < Formula
  desc "Interactive terminal app for learning Unix command-line tools"
  homepage "https://github.com/ShadabGhazali/cli-tutor"
  url "https://github.com/ShadabGhazali/cli-tutor/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "85326810f609c2bc6e9435cb79d094bcdbc620126ebde42509b84aee6db302f9"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cli-tutor --version 2>&1", 1)
  end
end
