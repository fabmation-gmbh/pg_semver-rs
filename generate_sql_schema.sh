#!/bin/bash

cargo pgx schema --release --out sql/_gen_full.sql
