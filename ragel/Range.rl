// -*- coding: utf-8 -*-
%%{
  machine range;

  main := |*
    'a'..'z'  => { emit("letter", data, tokens, ts, te); };
  *|;
}%%

import java.util.*;

public class Range {

  private static int cs;
  private static String p;
  private static String pe;

  public Range() { }

  %% write data;
  public void parse(byte[] data) {
    int eof = data.length;
    int p = 0, pe = data.length, te, ts, cs, act;
    List<String> tokens = new ArrayList<String>();
    %% write init;
    %% write exec;

    System.out.println(tokens);
  }

  public void emit(String token, byte[] data, List<String> tokens, int ts, int te) {
    byte[] output = new byte[te - ts + 1];
    System.arraycopy(data, ts, output, 0, te - ts);
    tokens.add(new String(output));
    System.out.println("processing: " + new String(data));
  }

  public static void main(String[] args) {
    new Range().parse("azb1".getBytes());
  }

}
