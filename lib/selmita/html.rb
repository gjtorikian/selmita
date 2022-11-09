module Selmita
  class HTML
    def initialize(html)
      @html = html
    end

    def rewrite
      selmita_rewrite
    end
  end
end
