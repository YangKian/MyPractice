package main

import (
	"html/template"
	"log"
	"os"
)

var tpl *template.Template

type sage struct {
	Name string
	Motto string
}

func init() {
	tpl = template.Must(template.ParseGlob("./template/deliver_compound_params/*.gohtml"))
}
func main() {
	sages := []string{"Gandhi", "MLk", "Buddha", "Jesus", "Muhammad"}
	err := tpl.Execute(os.Stdout, sages)
	if err != nil {
		log.Fatalln(err)
	}

	err = tpl.ExecuteTemplate(os.Stdout, "tpl_compound1.gohtml", sages)
	if err != nil {
		log.Fatalln(err)
	}

	sagesMp := map[string]string {
		"India": "Gandhi",
		"America": "MLK",
		"Meditate": "Buddha",
		"Love": "Jesus",
		"Prophet": "Muhammad",
	}

	err = tpl.ExecuteTemplate(os.Stdout, "tpl_compound.gohtml", sagesMp)
	if err != nil {
		log.Fatalln(err)
	}

	err = tpl.ExecuteTemplate(os.Stdout, "tpl_compound2.gohtml", sagesMp)
	if err != nil {
		log.Fatalln(err)
	}

	buddha := sage {
		Name: "Buddha",
		Motto: "The belief of no beliefs",
	}

	mlk := sage {
		Name: "Martin Luther King",
		Motto: "Hatred never ceases with hatred but with love alone is healed.",
	}

	err = tpl.ExecuteTemplate(os.Stdout, "tpl_compound3.gohtml", buddha)
	if err != nil {
		log.Fatalln(err)
	}

	sageSlice := []sage{buddha, mlk}
	err = tpl.ExecuteTemplate(os.Stdout, "tpl_compound4.gohtml", sageSlice)
	if err != nil {
		log.Fatalln(err)
	}
}
