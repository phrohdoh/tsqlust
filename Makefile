.PHONY: help build build-jsbindings force-publish-docs
.DEFAULT_GOAL: help

help:
	@echo "help               -> display this help"
	@echo "build              -> build tsqlust"
	@echo "build-jsbindings   -> build tsqlust with javascript bindings"
	@echo "force-publish-docs -> update gh-pages docs"

build:
	@cargo rustc --lib -v

build-jsbindings:
	@cargo rustc --lib -v --features jsbindings

force-publish-docs:
	@test -z "$(git status --porcelain)" || (echo "Cannot publish docs with a dirty working tree!"; exit 1)
	@cargo doc && \
	echo '<meta http-equiv=refresh content=0;url=tsqlust/index.html>' > target/doc/index.html && \
	ghp-import -n target/doc && \
	git push origin gh-pages