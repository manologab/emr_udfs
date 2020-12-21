import emr_udfs
import pandas as pd

DF_COLUMNS = ["advertisable_eid", "cookie", "type", "timestamp", "campaign_eid", "adgroup_eid"]


def test_pr_pageview_and_tos():
    data = [
        # 2 hits, 5s tos
        ("adv0", "cook0", "cli", 1000, "camp018PRO", "ad01"),
        ("adv0", "cook0", "pxl", 1005, "", ""),
        # 4 hits, 180s tos
        ("adv1", "cook0", "imp", 900, "camp018PRO", "ad01"),
        ("adv1", "cook0", "pxl", 1010, "", ""),
        ("adv1", "cook0", "pxl", 1015, "", ""),
        ("adv1", "cook0", "pxl", 1016, "", ""),  # ignored
        ("adv1", "cook0", "pxl", 1100, "", ""),
        ("adv1", "cook0", "pxl", 1190, "", ""),
        # 2 hits, 9000s tos
        ("adv0", "cook1", "cli", 1000, "camp018PRO", "ad01"),
        # ("adv0", "cook1", "pxl", 1010, "", ""),
        ("adv0", "cook1", "pxl", 10000, "", ""),

    ]
    df = pd.DataFrame.from_records(data=data, columns=DF_COLUMNS)
    df.info()
    df_r = emr_udfs.pr_pageview_and_tos(df, 1000)
    df_r = df_r.set_index("advertisable_eid")
    df_r.info()
    print(df_r)
    assert 2 == df_r.at["adv0", "page_views_2"]
    assert 0 == df_r.at["adv0", "page_views_3"]
    assert 0 == df_r.at["adv0", "page_views_4"]
    assert 0 == df_r.at["adv0", "page_views_5"]
    assert 0 == df_r.at["adv0", "page_views_6"]
    assert 0 == df_r.at["adv0", "page_views_7_or_more"]
    assert 1 == df_r.at["adv0", "time_on_site_0_to_1min"]
    assert 0 == df_r.at["adv0", "time_on_site_1_to_2min"]
    assert 0 == df_r.at["adv0", "time_on_site_2_to_3min"]
    assert 0 == df_r.at["adv0", "time_on_site_3_to_4min"]
    assert 0 == df_r.at["adv0", "time_on_site_4_to_5min"]
    assert 0 == df_r.at["adv0", "time_on_site_5_to_6min"]
    assert 0 == df_r.at["adv0", "time_on_site_6_to_7min"]
    assert 0 == df_r.at["adv0", "time_on_site_7_to_8min"]
    assert 0 == df_r.at["adv0", "time_on_site_8_to_9min"]
    assert 0 == df_r.at["adv0", "time_on_site_9_to_10min"]
    assert 1 == df_r.at["adv0", "time_on_site_10_or_more_min"]

    assert 0 == df_r.at["adv1", "page_views_2"]
    assert 0 == df_r.at["adv1", "page_views_3"]
    assert 1 == df_r.at["adv1", "page_views_4"]
    assert 0 == df_r.at["adv1", "page_views_5"]
    assert 0 == df_r.at["adv1", "page_views_6"]
    assert 0 == df_r.at["adv1", "page_views_7_or_more"]
    assert 0 == df_r.at["adv1", "time_on_site_0_to_1min"]
    assert 0 == df_r.at["adv1", "time_on_site_1_to_2min"]
    assert 1 == df_r.at["adv1", "time_on_site_2_to_3min"]
    assert 0 == df_r.at["adv1", "time_on_site_3_to_4min"]
    assert 0 == df_r.at["adv1", "time_on_site_4_to_5min"]
    assert 0 == df_r.at["adv1", "time_on_site_5_to_6min"]
    assert 0 == df_r.at["adv1", "time_on_site_6_to_7min"]
    assert 0 == df_r.at["adv1", "time_on_site_7_to_8min"]
    assert 0 == df_r.at["adv1", "time_on_site_8_to_9min"]
    assert 0 == df_r.at["adv1", "time_on_site_9_to_10min"]
    assert 0 == df_r.at["adv1", "time_on_site_10_or_more_min"]
