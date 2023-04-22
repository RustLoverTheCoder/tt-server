pub struct ApiChannelStatistics {
    growth_graph: Option<StatisticsGraph>,
    followers_graph: Option<StatisticsGraph>,
    mute_graph: Option<StatisticsGraph>,
    top_hours_graph: Option<StatisticsGraph>,
    interactions_graph: StatisticsGraph,
    views_by_source_graph: StatisticsGraph,
    new_followers_by_source_graph: StatisticsGraph,
    languages_graph: StatisticsGraph,
    followers: StatisticsOverviewItem,
    views_per_post: StatisticsOverviewItem,
    shares_per_post: StatisticsOverviewItem,
    enabled_notifications: StatisticsOverviewPercentage,
    recent_top_messages: Vec<StatisticsRecentMessage>,
}

pub struct StatisticsGraph {
    type_: String,
    zoom_token: Option<String>,
    label_formatter: String,
    tooltip_formatter: String,
    labels: Vec<String>,
    is_stacked: bool,
    is_percentage: Option<bool>,
    hide_caption: bool,
    has_second_y_axis: bool,
    minimap_range: MinimapRange,
    label_from_index: i32,
    label_to_index: i32,
    datasets: Vec<Dataset>,
}

pub struct MinimapRange {
    begin: i32,
    end: i32,
}

pub struct Dataset {
    name: String,
    color: String,
    values: Vec<i32>,
}

pub struct StatisticsOverviewItem {
    current: Option<i32>,
    change: Option<i32>,
    percentage: String,
}

pub struct StatisticsOverviewPercentage {
    percentage: String,
}

pub struct StatisticsRecentMessage {
    msg_id: i32,
    forwards: i32,
    views: i32,
}