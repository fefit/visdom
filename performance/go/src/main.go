package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

const (
	// LOOPTIMES :loop times
	LOOPTIMES = 200
	// NODECOUNT :node count
	NODECOUNT = 3000
)

// TotalInfo :total information
type TotalInfo struct {
	UsedTime float64
	Selector string
}

func getFileContent(file string) string {
	content, err := ioutil.ReadFile(file)
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}

func loadHTML() TotalInfo {
	content := getFileContent("../data/index.html")
	startTime := time.Now()
	for i := 0; i < LOOPTIMES; i++ {
		_, err := goquery.NewDocumentFromReader(strings.NewReader(content))
		if err != nil {
			log.Fatal(err)
		}
	}
	endTime := time.Now()
	usedTime := float64(endTime.Sub(startTime).Nanoseconds())
	return TotalInfo{
		Selector: "",
		UsedTime: (usedTime / 1.0e6 / float64(LOOPTIMES)),
	}
}

func execSelector(html *string, selector *string, init func(*goquery.Document) func(selector *string)) TotalInfo {
	doc, err := goquery.NewDocumentFromReader(strings.NewReader(*html))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("")
	fmt.Printf("Execute selector: %s", *selector)
	cb := init(doc)
	startTime := time.Now()
	for i := 0; i < LOOPTIMES; i++ {
		cb(selector)
	}
	endTime := time.Now()
	usedTime := float64(endTime.Sub(startTime).Nanoseconds())
	return TotalInfo{
		Selector: *selector,
		UsedTime: (usedTime / 1.0e6 / float64(LOOPTIMES)),
	}
}

func findID() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s%s</ul>", htmlItems, "<li id='target'></li>")
	selector := "#target"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.Find(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.Find(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findClass() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s%s</ul>", htmlItems, "<li CLASS='target'></li>")
	selector := ".target"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.Find(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.Find(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findName() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "dt"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.Find(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.Find(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findAttr() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd contenteditable></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "[contenteditable]"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.Find(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.Find(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findPrev() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "dd"
	init := func(doc *goquery.Document) func(*string) {
		dt := doc.Find("dl dt")
		fmt.Println()
		fmt.Printf("Find: %d", dt.PrevFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dt.PrevFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findPrevAll() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "dd"
	init := func(doc *goquery.Document) func(*string) {
		dt := doc.Find("dl dt")
		fmt.Println()
		fmt.Printf("Find: %d", dt.PrevAllFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dt.PrevAllFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findNext() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "dd"
	init := func(doc *goquery.Document) func(*string) {
		dt := doc.Find("dl dt")
		fmt.Println()
		fmt.Printf("Find: %d", dt.NextFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dt.NextFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func findNextAll() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := "dd"
	init := func(doc *goquery.Document) func(*string) {
		dt := doc.Find("dl dt")
		fmt.Println()
		fmt.Printf("Find: %d", dt.NextAllFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dt.NextAllFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func empty() TotalInfo {
	htmlItems := strings.Repeat("<li></li><li>a</li>", NODECOUNT/2)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":empty"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func contains() TotalInfo {
	htmlItems := strings.Repeat("<li></li><li>abcdefghijklmnopqrstuvwxyz&amp;abcdefghijklmnopqrstuvwxy</li>", NODECOUNT/2)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":contains('z&a')"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func firstChild() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":first-child"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func lastChild() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":last-child"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func firstOfType() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":first-of-type"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func lastOfType() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":last-of-type"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthChild() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-child(2n),:nth-child(3n),:nth-child(5n)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthChild10() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-child(10)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthChild2n5() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-child(2n + 5)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastChild() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastChild10() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-last-child(10)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastChild2n5() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-last-child(2n + 5)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthOfType() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-of-type(2n),:nth-of-type(3n)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthOfType10() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-of-type(10)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthOfType2n5() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-of-type(2n + 5)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastOfType() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-last-of-type(2n),:nth-last-of-type(3n)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastOfType10() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-last-of-type(10)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthLastOfType2n5() TotalInfo {
	htmlItems := strings.Repeat("<dt></dt><dd></dd>", NODECOUNT/2)
	html := fmt.Sprintf("<dl>%s</dl>", htmlItems)
	selector := ":nth-last-of-type(2n + 5)"
	init := func(doc *goquery.Document) func(*string) {
		dl := doc.Find("dl")
		fmt.Println()
		fmt.Printf("Find: %d", dl.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			dl.ChildrenFiltered(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func nthChildFind() TotalInfo {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-child(2n),:nth-child(3n),:nth-child(5n)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("Find: %d", ul.Find(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.Find(*selector)
		}
	}
	return execSelector(&html, &selector, init)
}

func main() {
	var totalInfos []TotalInfo
	totalInfos = append(totalInfos,
		loadHTML(),
		// findID(),
		// findClass(),
		// findName(),
		// findAttr(),
		// findPrev(),
		// findPrevAll(),
		// findNext(),
		// findNextAll(),
		// empty(),
		// contains(),
		// firstChild(),
		// lastChild(),
		// firstOfType(),
		// lastOfType(),
		// nthChild(),
		// nthChild10(),
		// nthChild2n5(),
		// nthLastChild(),
		// nthLastChild10(),
		// nthLastChild2n5(),
		// nthOfType(),
		// nthOfType10(),
		// nthOfType2n5(),
		// nthLastOfType(),
		// nthLastOfType10(),
		// nthLastOfType2n5(),
		// nthChildFind(),
	)
	fmt.Printf("%#v", totalInfos)
}
