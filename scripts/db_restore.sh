#!/bin/bash

psql -U classroom -c "DROP SCHEMA classroom CASCADE; CREATE SCHEMA classroom;"
cat classroom_dump_2021-07-26_17_50_20.sql | psql -U classroom
#psql -U classroom -c "alter users classroom with password 'Welc0meAuth1'"
