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

func execSelector(html *string, selector *string, init func(*goquery.Document) func(selector *string)) {
	doc, err := goquery.NewDocumentFromReader(strings.NewReader(*html))
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println("<tr>")
	fmt.Printf("<td>Execute selector: %s</td>", *selector)
	cb := init(doc)
	startTime := time.Now()
	for i := 0; i < LOOPTIMES; i++ {
		cb(selector)
	}
	endTime := time.Now()
	usedTime := float64(endTime.Sub(startTime).Nanoseconds())
	fmt.Println()
	fmt.Printf("<td>Elapsed: %.6f s, Average Time: %.6f ms</td>", usedTime/1.0e9, usedTime/1.0e6/float64(LOOPTIMES))
	fmt.Println()
	fmt.Print("</tr>")
}

func nthChild() {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-child(2n),:nth-child(3n),:nth-child(5n)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("<td>Find: %d</td>", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	execSelector(&html, &selector, init)
}

func nthLastChild() {
	htmlItems := strings.Repeat("<li></li>", NODECOUNT)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	selector := ":nth-last-child(2n),:nth-last-child(3n),:nth-last-child(5n)"
	init := func(doc *goquery.Document) func(*string) {
		ul := doc.Find("ul")
		fmt.Println()
		fmt.Printf("<td>Find: %d</td>", ul.ChildrenFiltered(selector).Length())
		fmt.Println()
		return func(selector *string) {
			ul.ChildrenFiltered(*selector)
		}
	}
	execSelector(&html, &selector, init)
}

func main() {
	nthChild()
	nthLastChild()
}
