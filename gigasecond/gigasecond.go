package gigasecond

import "time"

func AfterGigaSecond(start time.Time) time.Time {
	duration := 1000000000 * time.Second
	return start.Add(duration)
}
