
//#set_loc(1, "range.rl");

//#set_loc(5, "range.rl");



//#set_loc(9, "range.rs");
static _range_key_offsets: &[i8] = &[
    0,    0,    2
];

static _range_trans_keys: &[u8] = &[
   97,  122,    0
];

static _range_single_lengths: &[i8] = &[
    0,    0,    0
];

static _range_range_lengths: &[i8] = &[
    0,    1,    0
];

static _range_index_offsets: &[i8] = &[
    0,    0,    2
];

static _range_trans_targs: &[i8] = &[
    2,    0,    0,    0
];

static range_start: int = 1;
static range_first_final: int = 2;
static range_error: int = 0;

static range_en_main: int = 1;


//#set_loc(8, "range.rl");

//#set_loc(43, "range.rs");
fn main() -> () {
       let cs = range_start;

//#set_loc(9, "range.rl");

//#set_loc(50, "range.rs");
    let mut _klen: int;
    let mut _trans = 0;
    let mut _keys: int;
    let mut _goto_targ = 0;

    loop _goto: {
        match _goto_targ {
          0 => {
            if p == pe {
                _goto_targ = 4;
                loop _goto;
            }
            if cs == 0 {
                _goto_targ = 5;
                loop _goto;
            }
            _goto_targ = 1;
            loop _goto;
          }
          1 => {
            loop _match: {
                _keys = _range_key_offsets[cs] as int;
                _trans = _range_index_offsets[cs] as int;
                _klen = _range_single_lengths[cs] as int;
                if _klen > 0 {
                    let mut _lower: int = _keys;
                    let mut _mid: int;
                    let mut _upper: int = _keys + _klen - 1;
                    loop {
                        if _upper < _lower { break; }

                        _mid = _lower + ((_upper-_lower) >> 1);
                        if data[p] < _range_trans_keys[_mid] {
                            _upper = _mid - 1;
                        } else if data[p] > _range_trans_keys[_mid] {
                            _lower = _mid + 1;
                        } else {
                            _trans += (_mid - _keys);
                            break _match;
                        }
                    }
                    _keys += _klen;
                    _trans += _klen;
                }

                _klen = _range_range_lengths[cs] as int;
                if _klen > 0 {
                    let mut _lower = _keys;
                    let mut _mid: int;
                    let mut _upper = _keys + (_klen<<1) - 2;
                    loop {
                        if _upper < _lower { break; }

                        _mid = _lower + (((_upper-_lower) >> 1) & int::compl(1));
                        if data[p] < _range_trans_keys[_mid] {
                            _upper = _mid - 2;
                        } else if data[p] > _range_trans_keys[_mid+1] {
                            _lower = _mid + 2;
                        } else {
                            _trans += ((_mid - _keys)>>1);
                            break _match;
                        }
                    }
                    _trans += _klen;
                }
                break;
            }

            cs = _range_trans_targs[_trans] as int;

          _goto_targ = 2;
          loop;
        }
        2 => {
          if cs == 0 {
              _goto_targ = 5;
              loop _goto;
          }
          p += 1;
          if p != pe {
              _goto_targ = 1;
              loop _goto;
          }
        _goto_targ = 4;
        loop _goto;
        }
        4 => { }
        5 => { }
        _ => fail!(),      }
      break;
    }

//#set_loc(10, "range.rl");
}
