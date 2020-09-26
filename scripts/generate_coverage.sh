#!/bin/bash

# RUNNABLE='target/debug/ncas-4ff3770879fdcc77'
RUNNABLE=$1
kcov --include-path='./src/base/','./src/symbols/','./src/arithmetics/','./src/manipulation/','./src/exponential/','./src/trigonometrics/'   \
--verify                                                                \
target/cov                                                              \
$RUNNABLE
