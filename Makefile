.EXPORT_ALL_VARIABLES:

CMU_CONFIG = resources/config.toml
CMU_FILE_PATH = resources/changelog.md
CMU_AFTER_HEADING = unreleased
CMU_HEADING_LEVEL = 2
CMU_BASE_URL = https://github.com/ashmarchington/test

test:
	cargo test

local-run:
	cp ./resources/changelog.md ./resources/changelog.md.tmp
	cargo run ./resources/log.txt
	mv -f ./resources/changelog.md ./resources/changelog.run.md
	mv -f ./resources/changelog.md.tmp ./resources/changelog.md
