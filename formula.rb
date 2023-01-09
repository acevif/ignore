class Ignore < Formula
    desc "Manage your .gitignore file"
    homepage "https://github.com/acevif/ignore"
    url "https://github.com/acevif/ignore/archive/0.1.1.tar.gz"
    sha256 "086bfd73312536f49cc28860f3786f50a390be432113943a384b90d0cf458d93"
    license "Unlicense"
  
    def install
      bin.install "ignore"
    end
  
    test do
    end
  end
  