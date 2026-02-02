package com.mathfix;

import com.sun.jna.Library;
import com.sun.jna.Native;

public interface MathfixLib extends Library {

    MathfixLib INSTANCE =
            Native.load("mathfix",
                    MathfixLib.class);

    int add_u64(long a, long b, long[] out);
    int subtract_u64(long a, long b, long[] out);
    int multiply_u64(long a, long b, long[] out);
    int divide_u64(long a, long b, long[] out);
}
