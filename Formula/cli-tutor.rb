class CliTutor < Formula
  desc "Interactive terminal app for learning Unix command-line tools"
  homepage "https://github.com/ShadabGhazali/cli-tutor"
  url "https://github.com/ShadabGhazali/cli-tutor/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "5afa1718337c1d958f08d3d9bc13daee0ec56805fe48faebd18c0bd6cbbc84f2"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cli-tutor --version 2>&1", 1)
  end
end
