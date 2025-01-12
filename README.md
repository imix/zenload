# Zenload

A load tester for deterministic load scenarios.

## Motivation

zenload was created to have a tool to run for measuring the energy consumption of a defined load in different environments. If you find other uses for it, you can obviously use it for that.

It is not a stress testing tool, if it senses that it maxed out, it reports an error.

## Config file

zenload uses a config file in yaml format. With this file, zenarios (pun intended) can be described.

## Tests

Tests are labelled with the year they are written. This is done to allow to add new loads of the same type but which are scaled for newer machines.

### CPU

| test type | description |
| --------- | ----------- |
| int24 | Load generated with integer operations |

## About the name

zenload is for one the one hand pun on scenario-load and on the other hand it runs in a zen-like state, trying to do it's work in a repetitive and reproducible manner (sorry, don't know much about zen).
