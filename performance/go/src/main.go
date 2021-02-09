package main

import (
	"fmt"
	"log"
	"strings"
	"time"

	"github.com/PuerkitoBio/goquery"
)

func main() {
	loopNum := 200
	nodeCount := 3000
	htmlItems := strings.Repeat("<li></li>", nodeCount)
	html := fmt.Sprintf("<ul>%s</ul>", htmlItems)
	// Load the HTML document
	doc, err := goquery.NewDocumentFromReader(strings.NewReader(html))
	if err != nil {
		log.Fatal(err)
	}
	selector := ":nth-child(2n),:nth-child(3n),:nth-child(5n)"
	ul := doc.Find("ul")
	fmt.Printf("Html: <ul>{strings.Repeat('<li></li>', %d)}</ul>", nodeCount)
	fmt.Println()
	fmt.Printf("Query: ul.children('%s')", selector)
	fmt.Println()
	fmt.Printf("Find matched: %d", ul.ChildrenFiltered(selector).Length())
	fmt.Println()
	fmt.Printf("Execute %d times to get average time:", loopNum)
	startTime := time.Now()
	for i := 0; i < loopNum; i++ {
		ul.ChildrenFiltered(selector)
	}
	endTime := time.Now()
	usedTime := float64(endTime.Sub(startTime).Nanoseconds())

	fmt.Println()
	fmt.Printf("Elapsed: %.6f s, Average Time: %.6f ms", usedTime/1.0e9, usedTime/1.0e6/float64(loopNum))
}
