fn count_increments(report: &Vec<usize>) -> usize {
    report.windows(2).filter(|w| w[0] < w[1]).count()
}

#[cfg(test)]
mod tests {
    use crate::util::read_file;
    use super::*;

    #[test]
    fn should_count_increments() {
        // give
        let report = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        // when
        let increment_count = count_increments(&report);

        // then
        assert_eq!(increment_count, 7)
    }

    #[test]
    fn should_count_increments_task() {
        // give
        let report = read_file::<usize>("res/task_1.txt").unwrap();

        // when
        let increment_count = count_increments(&report);

        // then
        assert_eq!(increment_count, 1301)
    }
}
