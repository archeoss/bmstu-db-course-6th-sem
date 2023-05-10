.PHONY: coursework.pdf all clean

all: coursework.pdf

coursework.pdf: coursework.tex
	xelatex coursework.tex && bibtex coursework && xelatex coursework.tex && xelatex coursework.tex
	#latexmk -f -pdf -pdflatex="pdflatex -interaction=nonstopmode" -use-make coursework.tex

clean:
	latexmk -CA

