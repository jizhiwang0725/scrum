use crate::models::Recipe;
use crate::query::normalize_text;
use strsim::normalized_damerau_levenshtein; //Calculates a normalized score of the Damerau–Levenshtein algorithm

const ACCEPT_SCORE: f64 = 0.70;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MatchKind {
    Fuzzy,
    Contains,
    Exact,
}

fn rank_tit(tit: &str, query: &str) -> Option<(MatchKind, f64)> {
    let norm_tit = normalize_text(tit);
    let norm_query = normalize_text(query);

    if norm_tit == norm_query {
        return Some((MatchKind::Exact, 1.0)); //Exact Match
    }
    if norm_tit.contains(norm_query.as_str()) {
        return Some((MatchKind::Contains, 1.0)); //Contain Query
    }

    let score = normalized_damerau_levenshtein(&norm_tit, &query); //Score of fuzzy match

    if score >= ACCEPT_SCORE {
        Some((MatchKind::Fuzzy, score))
    } else {
        None
    }
}

pub(crate) fn search_recipes(rec: &[Recipe], query: &str) -> Vec<Recipe> {
    if normalize_text(query).is_empty() {
        let mut ans = rec.to_vec();
        ans.sort_by(|a, b| a.id.cmp(&b.id));
        return ans;
    }
    let mut res: Vec<(Recipe, MatchKind, f64)> = Vec::new();

    for u in rec {
        if let Some((kind, score)) = rank_tit(&u.title, query) {
            res.push((u.clone(), kind, score));
        }
    }

    res.sort_by(|(a_rec, a_kind, a_score), (b_rec, b_kind, b_score)| {
        b_kind
            .cmp(a_kind)
            .then_with(|| b_score.total_cmp(a_score))
            .then_with(|| a_rec.id.cmp(&b_rec.id))
    });

    let mut ans = Vec::new();

    for (rec, _, _) in res {
        ans.push(rec);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_recipe_with_one_typo() {
        let result = rank_tit("宫保鸡丁", "宫爆鸡丁");

        assert_eq!(result, Some((MatchKind::Fuzzy, 0.75)));
    }

    fn make_recipe(id: &str, title: &str) -> Recipe {
        Recipe {
            id: id.to_owned(),
            title: title.to_owned(),
            url: String::new(),
            author: String::new(),
            author_url: String::new(),
            description: String::new(),
            ingredients: Vec::new(),
            steps: Vec::new(),
            tips: String::new(),
            cover_image_urls: Vec::new(),
        }
    }
    #[test]
    fn sorts_matches_by_relevance() {
        let rec = vec![
            make_recipe("3", "宫爆鸡丁"),
            make_recipe("2", "家常宫保鸡丁"),
            make_recipe("1", "宫保鸡丁"),
            make_recipe("4", "巧克力蛋糕"),
        ];

        let ans = search_recipes(&rec, "宫保鸡丁");

        assert_eq!(ans.len(), 3);
        assert_eq!(ans[0].id, "1");
        assert_eq!(ans[1].id, "2");
        assert_eq!(ans[2].id, "3");
    }
}
