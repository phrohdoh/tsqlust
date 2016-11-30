force-publish-docs:
	@test -z "$(git status --porcelain)" || (echo "Cannot publish docs with a dirty working tree!"; exit 1)
	@cargo doc && \
	echo '<meta http-equiv=refresh content=0;url=tsqlust/index.html>' > target/doc/index.html && \
	ghp-import -n target/doc && \
	git push origin gh-pages