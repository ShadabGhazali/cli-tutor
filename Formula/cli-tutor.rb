class CliTutor < Formula
  desc "Interactive terminal app for learning Unix command-line tools"
  homepage "https://github.com/ShadabGhazali/cli-tutor"
  url "https://github.com/ShadabGhazali/cli-tutor/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "45c7f093c9bed2f6c65aa1815070b7635fcc4f06cade9cb56061f55a4b712619"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cli-tutor --version 2>&1", 1)
  end
end
