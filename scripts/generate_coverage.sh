#!/bin/bash

# RUNNABLE='target/debug/ncas-4ff3770879fdcc77'
RUNNABLE=$1
kcov --include-path='./src/'    \
--verify                        \
target/cov                      \
$RUNNABLE
