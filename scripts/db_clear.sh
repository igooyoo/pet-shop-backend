#!/bin/bash

psql -U classroom -c "DROP SCHEMA classroom CASCADE; DROP SCHEMA utils CASCADE; CREATE SCHEMA classroom; "
