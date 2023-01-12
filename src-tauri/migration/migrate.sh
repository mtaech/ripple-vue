#!/bin/bash
sea-orm-cli migrate generate -u --database-url sqlite:/home/huang/.config/ripple/db/ripple.db ${1}
