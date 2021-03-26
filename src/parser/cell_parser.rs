use super::{
    attribute_parser::*,
    base::{tstring, ws},
};
use crate::parser::base::{qfloat, qstring};
use crate::parser::group_parser::named_group_parser;
use crate::{LibRes, LibertyJson};
use nom::branch::permutation;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{opt, value},
    error::context,
    multi::{many0, many1},
    sequence::{delimited, preceded, tuple},
};
use serde_json::map::Map;

pub fn cell_parser(input: &str) -> LibRes<&str, (&str, LibertyJson)> {
    context(
        "Cell Parser",
        tuple((
            preceded(
                ws(tag("cell")),
                delimited(ws(tag("(")), tstring, ws(tag(")"))),
            ),
            delimited(
                ws(tag("{")),
                permutation((
                    many0(simple_attribute),
                    leakage_power_parser,
                    pgpin_parser,
                    pin_parser,
                    opt(test_cell_parser),
                    opt(ff_parser),
                )),
                ws(tag("}")),
            ),
        )),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        for attr_grp in (data.1).0 {
            json_data.insert(attr_grp.0.to_string(), attr_grp.1);
        }
        json_data.insert("leakage_power".into(), (data.1).1);
        json_data.insert("pgpin".into(), (data.1).2);
        json_data.insert("pin".into(), (data.1).3);
        // ignore testcell and ff temparally
        // if let Some(u) = (data.2).4 {
        //     json_data.insert("test_cell".into(), u);
        // }
        // if let Some(u) = (data.2).5 {
        //     json_data.insert("ff".into(), u);
        // }
        (res, (data.0, LibertyJson::from(json_data)))
    })
}

fn leakage_power_parser(input: &str) -> LibRes<&str, LibertyJson> {
    context(
        "Leakage Power Parser",
        many1(preceded(
            ws(tag("leakage_power()")),
            delimited(
                ws(tag("{")),
                tuple((
                    delimited(
                        tuple((ws(tag("related_pg_pin")), ws(tag(":")))),
                        qstring,
                        ws(tag(";")),
                    ),
                    delimited(
                        tuple((ws(tag("when")), ws(tag(":")))),
                        qstring,
                        ws(tag(";")),
                    ),
                    delimited(
                        tuple((ws(tag("value")), ws(tag(":")))),
                        qfloat,
                        ws(tag(";")),
                    ),
                )),
                ws(tag("}")),
            ),
        )),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        let mut result = Map::new();
        data.iter().for_each(|x| {
            let idx = format!("{},{}", x.0, x.1);
            let value = x.2;
            json_data.insert(idx, LibertyJson::from(value));
        });
        result.insert("leakage_power".into(), LibertyJson::from(json_data));
        (res, LibertyJson::from(result))
    })
}

fn pgpin_parser(input: &str) -> LibRes<&str, LibertyJson> {
    context(
        "PgPin Parser",
        many1(tuple((
            preceded(
                ws(tag("pg_pin")),
                delimited(ws(tag("(")), tstring, ws(tag(")"))),
            ),
            delimited(ws(tag("{")), many1(simple_attribute), ws(tag("}"))),
        ))),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        let mut result = Map::new();
        data.iter().for_each(|x| {
            let mut pg_pin_data = Map::new();
            for attr in &x.1 {
                pg_pin_data.insert(attr.0.into(), attr.1.clone());
            }
            json_data.insert(x.0.into(), LibertyJson::from(pg_pin_data));
        });
        result.insert("pg_pin".into(), LibertyJson::from(json_data));
        (res, LibertyJson::from(result))
    })
}

// regardless ff
fn ff_parser(input: &str) -> LibRes<&str, ()> {
    context(
        "FF parser",
        value(
            (),
            tuple((
                preceded(
                    ws(tag("ff")),
                    tuple((ws(tag("(")), take_until(")"), ws(tag(")")))),
                ),
                delimited(ws(tag("{")), take_until("}"), ws(tag("}"))),
            )),
        ),
    )(input)
}

// regardless test_cell
fn test_cell_parser(input: &str) -> LibRes<&str, ()> {
    context(
        "Test Cell Parser",
        value(
            (),
            preceded(
                ws(tag("test_cell()")),
                delimited(
                    ws(tag("{")),
                    many1(alt((testcell_pin_parser, ff_parser))),
                    ws(tag("}")),
                ),
            ),
        ),
    )(input)
}

fn testcell_pin_parser(input: &str) -> LibRes<&str, ()> {
    value(
        (),
        many1(tuple((
            preceded(
                ws(tag("pin")),
                delimited(ws(tag("(")), tstring, ws(tag(")"))),
            ),
            delimited(ws(tag("{")), many1(simple_attribute), ws(tag("}"))),
        ))),
    )(input)
}

fn pin_parser(input: &str) -> LibRes<&str, LibertyJson> {
    context(
        "Pin Parser",
        many1(tuple((
            preceded(
                ws(tag("pin")),
                delimited(ws(tag("(")), tstring, ws(tag(")"))),
            ),
            delimited(
                ws(tag("{")),
                tuple((
                    many1(simple_attribute),
                    internal_power_parser,
                    timing_parser,
                )),
                ws(tag("}")),
            ),
        ))),
    )(input)
    .map(|(res, data)| {
        let mut json_data = Map::new();
        data.iter().for_each(|x| {
            let mut one_pin_json = Map::new();
            let pin_data = &x.1;
            for attr in &pin_data.0 {
                one_pin_json.insert(attr.0.into(), attr.1.clone());
            }
            one_pin_json.insert("internal_power".into(), pin_data.1.clone());
            one_pin_json.insert("timing".into(), pin_data.2.clone());
            json_data.insert(x.0.into(), LibertyJson::from(one_pin_json));
        });
        (res, LibertyJson::from(json_data))
    })
}

fn internal_power_parser(input: &str) -> LibRes<&str, LibertyJson> {
    many1(preceded(
        ws(tag("internal_power()")),
        delimited(
            ws(tag("{")),
            many1(alt((simple_attribute, named_group_parser))),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = Vec::new();
        data.iter().for_each(|x| {
            let mut power_data = Map::new();
            for attr in x {
                power_data.insert(attr.0.into(), attr.1.clone());
            }
            json_data.push(power_data);
        });
        (res, LibertyJson::from(json_data))
    })
}

fn timing_parser(input: &str) -> LibRes<&str, LibertyJson> {
    many1(preceded(
        ws(tag("timing()")),
        delimited(
            ws(tag("{")),
            many1(alt((simple_attribute, named_group_parser))),
            ws(tag("}")),
        ),
    ))(input)
    .map(|(res, data)| {
        let mut json_data = Vec::new();
        data.iter().for_each(|x| {
            let mut timing_data = Map::new();
            for attr in x {
                timing_data.insert(attr.0.into(), attr.1.clone());
            }
            json_data.push(timing_data);
        });
        (res, LibertyJson::from(json_data))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_leakage_power() {
        let input_str = "    leakage_power() { 
      related_pg_pin : \"VDD\" ; 
      when : \"!A&!B&!CK&!SE&!SI\" ; 
      value : \"0.0028893804\" ; 
    }

    leakage_power() { 
      related_pg_pin : \"VDD\" ; 
      when : \"!A&!B&!CK&!SE&SI\" ; 
      value : \"0.00340529632\" ; 
    }";
        let (_, _) = leakage_power_parser(input_str).unwrap();
    }

    #[test]
    fn test_pg_pin_parser() {
        let input_str = "    pg_pin(VDD) { 
      voltage_name : VDD ; 
      pg_type : primary_power ; 
    }

    pg_pin(VSS) { 
      voltage_name : VSS ; 
      pg_type : primary_ground ; 
    }";
        let (_, _) = pgpin_parser(input_str).unwrap();
    }

    #[test]
    fn test_internal_power_parser() {
        let input_str = "      internal_power() { 
        when : \"B&!CK&SE&SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"0.000174778, 0.000176158, 0.000176025, 0.000176212, \\
                 0.000175917, 0.000175715, 0.000175441\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"-5.50402e-05, -5.53276e-05, -5.04217e-05, \\
                 -4.65125e-05, -4.63012e-05, -4.60229e-05, -4.56864e-05\");
        }
      }";
        let (_, _) = internal_power_parser(input_str).unwrap();
    }

    #[test]
    fn test_timing_parser() {
        let input_str = "      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_B_AND_NOT_SE_AND_NOT_SI == 1'b1\" ; 
        timing_type : hold_rising ; 
        when : \"B&!SE&!SI\" ; 

        fall_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.021892, 0.00577376, 0.026052\",\\
                 \"-0.0788896, -0.0556158, -0.0367075\",\\
                 \"-0.162826, -0.152302, -0.152124\");
        }

        rise_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.02766, -0.0146162, -0.021688\",\\
                 \"-0.0589196, -0.0488958, -0.0561175\",\\
                 \"-0.0472362, -0.0455724, -0.0711342\");
        }
      }";
        let (_, _) = timing_parser(input_str).unwrap();
    }

    #[test]
    fn test_testcell() {
        let input_str = "    test_cell() { 

      pin(SI) { 
        direction : input ; 
        signal_type : test_scan_in ; 
      }

      pin(SE) { 
        direction : input ; 
        signal_type : test_scan_enable ; 
      }

      pin(A) { 
        direction : input ; 
      }

      pin(CK) { 
        direction : input ; 
      }

      pin(B) { 
        direction : input ; 
      }

      ff(IQ,IQN) { 
        clocked_on : \"CK\" ; 
        next_state : \"(B  A)\" ; 
      }

      pin(QN) { 
        direction : output ; 
        function : \"IQN\" ; 
        signal_type : test_scan_out_inverted ; 
      }
    }";
        let (_, _) = test_cell_parser(input_str).unwrap();
    }
    #[test]
    fn test_ff() {
        let input_str = "        ff(IQ,IQN) { 
      clocked_on : \"CK\" ; 
      next_state : \"(SE SI) + (!SE ((B  A)))\" ; 
    }
";
        let (_, _) = ff_parser(input_str).unwrap();
    }

    #[test]
    fn test_cell() {
        let input_str = "  cell(A2SDFFQN_X0P5M_A9TL40) { 
    area : 5.5062 ; 
    cell_footprint : A2SDFFQN_X0P5M ; 

    leakage_power() { 
      related_pg_pin : \"VDD\" ; 
      when : \"!A&!B&!CK&!SE&!SI\" ; 
      value : \"0.0028893804\" ; 
    }

    leakage_power() { 
      related_pg_pin : \"VDD\" ; 
      when : \"!A&!B&!CK&!SE&SI\" ; 
      value : \"0.00340529632\" ; 
    }

        pg_pin(VDD) { 
      voltage_name : VDD ; 
      pg_type : primary_power ; 
    }

    pg_pin(VSS) { 
      voltage_name : VSS ; 
      pg_type : primary_ground ; 
    }
    pin(A) { 
      capacitance : 0.000481542 ; 
      direction : input ; 
      fall_capacitance : 0.000484208 ; 
      input_voltage : default ; 
      max_transition : 1.236 ; 
      nextstate_type : data ; 
      related_ground_pin : VSS ; 
      related_power_pin : VDD ; 
      rise_capacitance : 0.000478877 ; 
      direction : output ; 
      function : \"IQN\" ; 
      max_capacitance : 0.0703029 ; 
      max_transition : 1.236 ; 
      min_capacitance : 0.0001 ; 
      output_voltage : default ; 
      related_ground_pin : VSS ; 
      related_power_pin : VDD ; 
      power_down_function : \"!VDD + VSS\" ; 

      internal_power() { 
        when : \"!B&!CK&!SE&!SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"0.000375434, 0.000376121, 0.000385726, 0.000389842, \\
                 0.000392275, 0.000392902, 0.000393199\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"-0.000107745, -0.000110585, -0.000111607, \\
                 -0.000112191, -0.000112566, -0.000112543, -0.000112614\");
        }
      }

      internal_power() { 
        when : \"!B&!CK&!SE&SI\" ; 

        fall_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"0.000375384, 0.00037611, 0.000385724, 0.000389849, \\
                 0.000392275, 0.0003929, 0.000393197\");
        }

        rise_power(pwr_tin_7) { 
          index_1(\"0.0021, 0.0242957, 0.102208, 0.250991, 0.48279, \\
                  0.808029, 1.236\");
          values(\"-0.000107748, -0.00011059, -0.000111612, -0.000112196, \\
                 -0.000112572, -0.000112549, -0.000112621\");
        }
      }
            internal_power() { 
        related_pin : \"CK\" ; 
        when : \"!A&!B&SE&!SI\" ; 

        fall_power(pwr_tin_oload_7x7) { 
          index_1(\"0.0021, 0.013179, 0.0520686, 0.126334, 0.242036, \\
                  0.404379, 0.618\");
          index_2(\"0.0001, 0.00136283, 0.00579563, 0.0142607, \\
                  0.0274489, 0.0459534, 0.0703029\");
          values(\"0.00215548, 0.00214066, 0.00210645, 0.00203794, 0.00192852, 0.00175905, 0.00151271\",\\
                 \"0.00215581, 0.00214109, 0.00210717, 0.00204007, 0.0019297, 0.00176225, 0.00151965\",\\
                 \"0.0021614, 0.00214651, 0.00211278, 0.00204261, 0.00193391, 0.00176368, 0.00151591\",\\
                 \"0.00217983, 0.00216591, 0.00213311, 0.00206187, 0.00195259, 0.00177936, 0.00153506\",\\
                 \"0.00219762, 0.00218872, 0.00214699, 0.00207686, 0.00196245, 0.00179173, 0.00155095\",\\
                 \"0.00222725, 0.00221979, 0.00218596, 0.00210796, 0.00199559, 0.00182423, 0.00158379\",\\
                 \"0.0022589, 0.00224961, 0.00225075, 0.0021385, 0.00202494, 0.00185802, 0.00161799\");
        }

        rise_power(pwr_tin_oload_7x7) { 
          index_1(\"0.0021, 0.013179, 0.0520686, 0.126334, 0.242036, \\
                  0.404379, 0.618\");
          index_2(\"0.0001, 0.00136283, 0.00579563, 0.0142607, \\
                  0.0274489, 0.0459534, 0.0703029\");
          values(\"0.00215548, 0.00214066, 0.00210645, 0.00203794, 0.00192852, 0.00175905, 0.00151271\",\\
                 \"0.00215581, 0.00214109, 0.00210717, 0.00204007, 0.0019297, 0.00176225, 0.00151965\",\\
                 \"0.0021614, 0.00214651, 0.00211278, 0.00204261, 0.00193391, 0.00176368, 0.00151591\",\\
                 \"0.00217983, 0.00216591, 0.00213311, 0.00206187, 0.00195259, 0.00177936, 0.00153506\",\\
                 \"0.00219762, 0.00218872, 0.00214699, 0.00207686, 0.00196245, 0.00179173, 0.00155095\",\\
                 \"0.00222725, 0.00221979, 0.00218596, 0.00210796, 0.00199559, 0.00182423, 0.00158379\",\\
                 \"0.0022589, 0.00224961, 0.00225075, 0.0021385, 0.00202494, 0.00185802, 0.00161799\");
        }
      }



      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_B_AND_NOT_SE_AND_NOT_SI == 1'b1\" ; 
        timing_type : hold_rising ; 
        when : \"B&!SE&!SI\" ; 

        fall_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.021892, 0.00577376, 0.026052\",\\
                 \"-0.0788896, -0.0556158, -0.0367075\",\\
                 \"-0.162826, -0.152302, -0.152124\");
        }

        rise_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.02766, -0.0146162, -0.021688\",\\
                 \"-0.0589196, -0.0488958, -0.0561175\",\\
                 \"-0.0472362, -0.0455724, -0.0711342\");
        }
      }
      timing() { 
        related_pin : \"CK\" ; 
        sdf_cond : \"ENABLE_B_AND_NOT_SE_AND_NOT_SI == 1'b1\" ; 
        timing_type : hold_rising ; 
        when : \"B&!SE&!SI\" ; 

        fall_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.021892, 0.00577376, 0.026052\",\\
                 \"-0.0788896, -0.0556158, -0.0367075\",\\
                 \"-0.162826, -0.152302, -0.152124\");
        }

        rise_constraint(cnst_ctin_rtin_3x3) { 
          index_1(\"0.0021, 0.275679, 1.236\");
          index_2(\"0.0021, 0.138657, 0.618\");
          values(\"-0.02766, -0.0146162, -0.021688\",\\
                 \"-0.0589196, -0.0488958, -0.0561175\",\\
                 \"-0.0472362, -0.0455724, -0.0711342\");
        }
      }
  }
      test_cell() { 

      pin(SI) { 
        direction : input ; 
        signal_type : test_scan_in ; 
      }

      pin(SE) { 
        direction : input ; 
        signal_type : test_scan_enable ; 
      }

      pin(A) { 
        direction : input ; 
      }

      pin(CK) { 
        direction : input ; 
      }

      pin(B) { 
        direction : input ; 
      }

      ff(IQ,IQN) { 
        clocked_on : \"CK\" ; 
        next_state : \"(B  A)\" ; 
      }

      pin(QN) { 
        direction : output ; 
        function : \"IQN\" ; 
        signal_type : test_scan_out_inverted ; 
      }
    }

    ff(IQ,IQN) { 
      clocked_on : \"CK\" ; 
      next_state : \"(SE SI) + (!SE ((B  A)))\" ; 
    }
  }

    ";
        let (_, _) = cell_parser(input_str).unwrap();
    }
}
