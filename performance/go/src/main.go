package main

import (
	"fmt"
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

type TotalInfo struct {
	UsedTime float64
	Selector string
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

func findId() TotalInfo {
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
	html := fmt.Sprintf("<ul>%s%s</ul>", htmlItems, "<li class='target'></li>")
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

func main() {
	var totalInfos []TotalInfo
	totalInfos = append(totalInfos,
		nthChild(),
		nthLastChild(),
		nthOfType(),
		nthLastOfType(),
		nthChildFind(),
		findId(),
		findClass(),
		findAttr(),
	)
	fmt.Printf("%#v", totalInfos)
}
