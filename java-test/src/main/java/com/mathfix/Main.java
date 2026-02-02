package com.mathfix;

public class Main {
    public static void main(String[] args) {

        long[] res_soma = new long[1];
        long[] res_subtracao = new long[1];
        long[] res_multiplicacao = new long[1];
        long[] res_divisao = new long[1];

        MathfixLib.INSTANCE.add_u64(100,200, res_soma);
        MathfixLib.INSTANCE.subtract_u64(1000,200, res_subtracao);
        MathfixLib.INSTANCE.multiply_u64(100,200, res_multiplicacao);
        MathfixLib.INSTANCE.divide_u64(1000,200, res_divisao);

        System.out.println("Soma: "+res_soma[0]);
        System.out.println("Subtracao: "+res_subtracao[0]);
        System.out.println("Multiplicacao: "+res_multiplicacao[0]);
        System.out.println("Divisao: "+res_divisao[0]);
    }
}