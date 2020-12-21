use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{IntoPyDict};
use pyo3::conversion::{IntoPy};
use numpy::{PyArray1, IntoPyArray};
use itertools::izip;
use std::iter::Iterator;
use std::collections::HashMap;

/// Returns an iterator for a Pandas String column
fn iter_col(col: &PyAny) -> impl Iterator<Item=&str>{
    col.iter().unwrap().map(|n| n.and_then(PyAny::extract::<&str>).unwrap())
}


#[derive(PartialEq)]
enum LineType {
    CLI,
    PXL,
    IMP,
    OTHER
}

impl LineType {
    fn from_str(linetype: &str) -> LineType {
        match linetype {
            "cli" => LineType::CLI,
            "pxl" => LineType::PXL,
            "imp" => LineType::IMP,
            _ => LineType::OTHER
        }
    }
}

#[derive(Default, Debug)]
struct ResultItem {
    page_views_2: u64,
    page_views_3: u64,
    page_views_4: u64,
    page_views_5: u64,
    page_views_6: u64,
    page_views_7_or_more: u64,
    tos_01: u64,
    tos_12: u64,
    tos_23: u64,
    tos_34: u64,
    tos_45: u64,
    tos_56: u64,
    tos_67: u64,
    tos_78: u64,
    tos_89: u64,
    tos_910: u64,
    tos_10_or_more: u64,
}

#[derive(Debug)]
struct Results<'a> {
    results: HashMap<&'a str, ResultItem>
}

impl<'a> Results<'a> {
    fn new() -> Results<'a> {
        Results {
            results: HashMap::new()
        }
    }
    fn add_cookie(&mut self, adv: &'a str, hits: i64, tos: i64) {
        if hits < 2 {
            return;
        }

        let item: &mut ResultItem = self.results.entry(adv).or_insert_with(|| Default::default());
        match hits {
            1 => (),
            2 => item.page_views_2 += 1,
            3 => item.page_views_3 += 1,
            4 => item.page_views_4 += 1,
            5 => item.page_views_5 += 1,
            6 => item.page_views_6 += 1,
            _ => item.page_views_7_or_more += 1,
        }
        match (tos-1) / 60 {
            0 => item.tos_01 += 1,
            1 => item.tos_12 += 1,
            2 => item.tos_23 += 1,
            3 => item.tos_34 += 1,
            4 => item.tos_45 += 1,
            5 => item.tos_56 += 1,
            6 => item.tos_67 += 1,
            7 => item.tos_78 += 1,
            8 => item.tos_89 += 1,
            9 => item.tos_910 += 1,
            _ => item.tos_10_or_more += 1,
        }
    }

    fn to_dataframe<'b>(&self, py: Python<'b>) -> PyResult<&'b PyAny> {
        let num_items = self.results.len();
        let mut advs: Vec<&str> = Vec::with_capacity(num_items);
        let mut page_views_2: Vec<u64> = Vec::with_capacity(num_items);
        let mut page_views_3: Vec<u64> = Vec::with_capacity(num_items);
        let mut page_views_4: Vec<u64> = Vec::with_capacity(num_items);
        let mut page_views_5: Vec<u64> = Vec::with_capacity(num_items);
        let mut page_views_6: Vec<u64> = Vec::with_capacity(num_items);
        let mut page_views_7_or_more: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_01: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_12: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_23: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_34: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_45: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_56: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_67: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_78: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_89: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_910: Vec<u64> = Vec::with_capacity(num_items);
        let mut tos_10_or_more: Vec<u64> = Vec::with_capacity(num_items);
        for (adv, item) in self.results.iter() {
            advs.push(*adv);
            page_views_2.push(item.page_views_2);
            page_views_3.push(item.page_views_3);
            page_views_4.push(item.page_views_4);
            page_views_5.push(item.page_views_5);
            page_views_6.push(item.page_views_6);
            page_views_7_or_more.push(item.page_views_7_or_more);
            tos_01.push(item.tos_01);
            tos_12.push(item.tos_12);
            tos_23.push(item.tos_23);
            tos_34.push(item.tos_34);
            tos_45.push(item.tos_45);
            tos_56.push(item.tos_56);
            tos_67.push(item.tos_67);
            tos_78.push(item.tos_78);
            tos_89.push(item.tos_89);
            tos_910.push(item.tos_910);
            tos_10_or_more.push(item.tos_10_or_more);
        }
        let r = [
            ("page_views_2", page_views_2.into_pyarray(py)),
            ("page_views_3", page_views_3.into_pyarray(py)),
            ("page_views_4", page_views_4.into_pyarray(py)),
            ("page_views_5", page_views_5.into_pyarray(py)),
            ("page_views_6", page_views_6.into_pyarray(py)),
            ("page_views_7_or_more", page_views_7_or_more.into_pyarray(py)),
            ("time_on_site_0_to_1min", tos_01.into_pyarray(py)),
            ("time_on_site_1_to_2min", tos_12.into_pyarray(py)),
            ("time_on_site_2_to_3min", tos_23.into_pyarray(py)),
            ("time_on_site_3_to_4min", tos_34.into_pyarray(py)),
            ("time_on_site_4_to_5min", tos_45.into_pyarray(py)),
            ("time_on_site_5_to_6min", tos_56.into_pyarray(py)),
            ("time_on_site_6_to_7min", tos_67.into_pyarray(py)),
            ("time_on_site_7_to_8min", tos_78.into_pyarray(py)),
            ("time_on_site_8_to_9min", tos_89.into_pyarray(py)),
            ("time_on_site_9_to_10min", tos_910.into_pyarray(py)),
            ("time_on_site_10_or_more_min", tos_10_or_more.into_pyarray(py)),
        ].into_py_dict(py);
        r.set_item("advertisable_eid", advs.into_py(py))?;
        let pandas = PyModule::import(py, "pandas")?;
        Ok(pandas.call1("DataFrame", (r,))?)
    }
}

#[pyfunction]
fn pr_pageview_and_tos<'a>(py: Python<'a>, df: &PyAny, day_timestamp: i64) -> PyResult<&'a PyAny> {
    let timestamp_col_np: &PyArray1<i64> = df.getattr("timestamp")?.call_method0("to_numpy")?.extract()?;
    let timestamp_col_ro = timestamp_col_np.readonly();
    let timestamp_col: &[i64] = timestamp_col_ro.as_slice()?;
    let adv_col = df.getattr("advertisable_eid")?;
    let cookie_col = df.getattr("cookie")?;
    let type_col = df.getattr("type")?;
    let campaign_eid = df.getattr("campaign_eid")?;
    let adgroup_eid = df.getattr("adgroup_eid")?;

    let mut results = Results::new();
    let mut this_cookie: &str = "__none__";
    let mut this_adv: &str = "__none__";
    let mut hits: i64 = 0;
    let mut is_prospecting = false;
    let mut no_first_visitor = false;
    let mut first_hit_tst: i64 = 0;
    let mut last_hit_tst: i64 = 0;
    let mut last_imp_tst: i64 = 0;

    use LineType::{CLI, PXL, IMP, OTHER};

    for (timestamp, linetype, cookie, advertisable,
         campaign_eid, adgroup_eid) in izip!(timestamp_col.iter(),
                                             iter_col(type_col),
                                             iter_col(cookie_col),
                                             iter_col(adv_col),
                                             iter_col(campaign_eid),
                                             iter_col(adgroup_eid)) {
        println!("{:?}, {:?}, {:?}, {:?}", timestamp, linetype, cookie, advertisable);
        if this_cookie != cookie  || this_adv != advertisable {
            //TODO; finish previous cookie
            if is_prospecting && !no_first_visitor {
                results.add_cookie(this_adv, hits, last_hit_tst-first_hit_tst);
            }

            this_cookie = cookie;
            this_adv = advertisable;
            hits = 0;
            is_prospecting = false;
            first_hit_tst = 0;
            last_hit_tst = 0;
            last_imp_tst = 0;
            no_first_visitor = false;
        }

        let linetype = LineType::from_str(linetype);
        let timestamp = *timestamp;

        // ignore cookies with previous visits or unrelated events
        if no_first_visitor || linetype == OTHER {
            continue;
        }

        // first visit must be on the measurement day
        if (linetype == CLI || linetype == PXL)
            && timestamp < day_timestamp {
            no_first_visitor = true;
            continue;
        }

        // look only prospecting cookies
        if (linetype == CLI || linetype == IMP) &&
            (campaign_eid.ends_with("8PRO") || adgroup_eid.ends_with("8PRO")) {
            is_prospecting = true;
        }

        if linetype == IMP {
            last_imp_tst = timestamp;
        } else if linetype == CLI {
            if hits == 0 {
                hits += 1;
                first_hit_tst = timestamp;
                last_hit_tst = timestamp;
            }
        } else if linetype == PXL {
            if hits == 0 {
                if timestamp - last_imp_tst < 72*60*60 {
                    hits += 1;
                    first_hit_tst = timestamp;
                    last_hit_tst = timestamp;
                }
            } else {
                if timestamp - last_hit_tst >= 3 {
                    hits += 1;
                    last_hit_tst = timestamp;
                }
            }
        }
    }
    if is_prospecting && !no_first_visitor {
        results.add_cookie(this_adv, hits, last_hit_tst-first_hit_tst);
    }
    println!("Results: {:?}", results);
    results.to_dataframe(py)
}



/// A Python module implemented in Rust.
#[pymodule]
fn emr_udfs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pr_pageview_and_tos, m)?)?;

    Ok(())
}
