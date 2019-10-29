#!/usr/bin/env bash

~/build/ghidra_9.1-BETA_DEV_20190923/ghidra_9.1-BETA_DEV/support/analyzeHeadless ./ghidra-smb2 smb2 -process '*.o' -recursive -postScript StrandMatchExport.java
