#!/bin/bash

pg_dump -c -U auth >  classroom_dump_`date +%Y-%m-%d"_"%H_%M_%S`.sql