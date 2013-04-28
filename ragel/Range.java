
// line 1 "Range.rl"
// -*- coding: utf-8 -*-

// line 8 "Range.rl"


import java.util.*;

public class Range {

  private static int cs;
  private static String p;
  private static String pe;

  public Range() { }

  
// line 20 "Range.java"
private static byte[] init__range_actions_0()
{
	return new byte [] {
	    0,    1,    0,    1,    1,    1,    2
	};
}

private static final byte _range_actions[] = init__range_actions_0();


private static byte[] init__range_key_offsets_0()
{
	return new byte [] {
	    0,    0
	};
}

private static final byte _range_key_offsets[] = init__range_key_offsets_0();


private static char[] init__range_trans_keys_0()
{
	return new char [] {
	   97,  122,    0
	};
}

private static final char _range_trans_keys[] = init__range_trans_keys_0();


private static byte[] init__range_single_lengths_0()
{
	return new byte [] {
	    0,    0
	};
}

private static final byte _range_single_lengths[] = init__range_single_lengths_0();


private static byte[] init__range_range_lengths_0()
{
	return new byte [] {
	    0,    1
	};
}

private static final byte _range_range_lengths[] = init__range_range_lengths_0();


private static byte[] init__range_index_offsets_0()
{
	return new byte [] {
	    0,    0
	};
}

private static final byte _range_index_offsets[] = init__range_index_offsets_0();


private static byte[] init__range_trans_targs_0()
{
	return new byte [] {
	    1,    0,    0
	};
}

private static final byte _range_trans_targs[] = init__range_trans_targs_0();


private static byte[] init__range_trans_actions_0()
{
	return new byte [] {
	    5,    0,    0
	};
}

private static final byte _range_trans_actions[] = init__range_trans_actions_0();


private static byte[] init__range_to_state_actions_0()
{
	return new byte [] {
	    0,    1
	};
}

private static final byte _range_to_state_actions[] = init__range_to_state_actions_0();


private static byte[] init__range_from_state_actions_0()
{
	return new byte [] {
	    0,    3
	};
}

private static final byte _range_from_state_actions[] = init__range_from_state_actions_0();


static final int range_start = 1;
static final int range_first_final = 1;
static final int range_error = 0;

static final int range_en_main = 1;


// line 21 "Range.rl"
  public void parse(byte[] data) {
    int eof = data.length;
    int p = 0, pe = data.length, te, ts, cs, act;
    List<String> tokens = new ArrayList<String>();
    
// line 134 "Range.java"
	{
	cs = range_start;
	ts = -1;
	te = -1;
	act = 0;
	}

// line 26 "Range.rl"
    
// line 144 "Range.java"
	{
	int _klen;
	int _trans = 0;
	int _acts;
	int _nacts;
	int _keys;
	int _goto_targ = 0;

	_goto: while (true) {
	switch ( _goto_targ ) {
	case 0:
	if ( p == pe ) {
		_goto_targ = 4;
		continue _goto;
	}
	if ( cs == 0 ) {
		_goto_targ = 5;
		continue _goto;
	}
case 1:
	_acts = _range_from_state_actions[cs];
	_nacts = (int) _range_actions[_acts++];
	while ( _nacts-- > 0 ) {
		switch ( _range_actions[_acts++] ) {
	case 1:
// line 1 "NONE"
	{ts = p;}
	break;
// line 173 "Range.java"
		}
	}

	_match: do {
	_keys = _range_key_offsets[cs];
	_trans = _range_index_offsets[cs];
	_klen = _range_single_lengths[cs];
	if ( _klen > 0 ) {
		int _lower = _keys;
		int _mid;
		int _upper = _keys + _klen - 1;
		while (true) {
			if ( _upper < _lower )
				break;

			_mid = _lower + ((_upper-_lower) >> 1);
			if ( data[p] < _range_trans_keys[_mid] )
				_upper = _mid - 1;
			else if ( data[p] > _range_trans_keys[_mid] )
				_lower = _mid + 1;
			else {
				_trans += (_mid - _keys);
				break _match;
			}
		}
		_keys += _klen;
		_trans += _klen;
	}

	_klen = _range_range_lengths[cs];
	if ( _klen > 0 ) {
		int _lower = _keys;
		int _mid;
		int _upper = _keys + (_klen<<1) - 2;
		while (true) {
			if ( _upper < _lower )
				break;

			_mid = _lower + (((_upper-_lower) >> 1) & ~1);
			if ( data[p] < _range_trans_keys[_mid] )
				_upper = _mid - 2;
			else if ( data[p] > _range_trans_keys[_mid+1] )
				_lower = _mid + 2;
			else {
				_trans += ((_mid - _keys)>>1);
				break _match;
			}
		}
		_trans += _klen;
	}
	} while (false);

	cs = _range_trans_targs[_trans];

	if ( _range_trans_actions[_trans] != 0 ) {
		_acts = _range_trans_actions[_trans];
		_nacts = (int) _range_actions[_acts++];
		while ( _nacts-- > 0 )
	{
			switch ( _range_actions[_acts++] )
			{
	case 2:
// line 6 "Range.rl"
	{te = p+1;{ emit("letter", data, tokens, ts, te); }}
	break;
// line 239 "Range.java"
			}
		}
	}

case 2:
	_acts = _range_to_state_actions[cs];
	_nacts = (int) _range_actions[_acts++];
	while ( _nacts-- > 0 ) {
		switch ( _range_actions[_acts++] ) {
	case 0:
// line 1 "NONE"
	{ts = -1;}
	break;
// line 253 "Range.java"
		}
	}

	if ( cs == 0 ) {
		_goto_targ = 5;
		continue _goto;
	}
	if ( ++p != pe ) {
		_goto_targ = 1;
		continue _goto;
	}
case 4:
case 5:
	}
	break; }
	}

// line 27 "Range.rl"

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
