class CliTutor < Formula
  desc "Interactive terminal app for learning Unix command-line tools"
  homepage "https://github.com/ShadabGhazali/cli-tutor"
  url "https://github.com/ShadabGhazali/cli-tutor/archive/refs/tags/v0.1.1.tar.gz"
  sha256 "6234d48f8522c87f783d8032386b0201e40bc1ad966b6e93a034514b624e776f"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--locked", "--root", prefix, "--path", "."
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/cli-tutor --version 2>&1", 1)
  end
end
