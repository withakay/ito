SLIDES_MD := ai-assisted-engineering-patterns-march-2026.md
SLIDES_HTML := slides.html
PLAYFUL_MD := ai-assisted-engineering-patterns-march-2026-playful.md
PLAYFUL_HTML := slides-playful.html
NEON_MD := ai-assisted-engineering-patterns-march-2026-neon.md
NEON_HTML := slides-neon.html
NEON_THEME := themes/neon-grid.css
DIAGRAM_MMD := ai-assisted-engineering-patterns-diagram.mmd
DIAGRAM_SVG := ai-assisted-engineering-patterns-diagram.svg
MERMAID_CONFIG := mermaid.config.json

.PHONY: slides slides-playful slides-neon diagram open open-playful open-neon

slides:
	npx @marp-team/marp-cli@latest $(SLIDES_MD) -o $(SLIDES_HTML)

slides-playful:
	npx @marp-team/marp-cli@latest $(PLAYFUL_MD) -o $(PLAYFUL_HTML)

slides-neon:
	npx @marp-team/marp-cli@latest --theme-set $(NEON_THEME) --theme neon-grid $(NEON_MD) -o $(NEON_HTML)

diagram:
	npx -p @mermaid-js/mermaid-cli mmdc -i $(DIAGRAM_MMD) -o $(DIAGRAM_SVG) -c $(MERMAID_CONFIG) -b white

open: slides
	open $(SLIDES_HTML)

open-playful: slides-playful
	open $(PLAYFUL_HTML)

open-neon: slides-neon
	open $(NEON_HTML)
