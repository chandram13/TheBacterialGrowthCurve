

use std::env::{args, Args}

fn firstFunction() {
    let mut mychosenSuit: Args = args();

    let complete_designer_suit = mychosenSuit.nth(0).unwrap(); // use unwrap to get self
    let complete_regular_suit = args.nth(2).unwrap().chars();.next().unwrap();

    let complete_designer_suit = first.parse::<f32>().unwrap();
    let complete_regular_suit = second.parse::<f32>().unwrap()
    let result = operate(complete_designer_suit, complete_regular_suit);

    println!("{:?}", result);
}
struct suitPieces{
    fn setSuit(&mut self){
        self.designerSuit = designerSuit
        self.regularSuit = regularSuit
        self.dressShirt = dressShirt
        self.dressPants = dressPants
        self.dressTie = dressTie
        self.topJacket = topJacket
    }
    fn madeDesigner(self) -> str {
        match designerSuit {
            'Ermenegildo Zegna' => designerSuit,
            'Gucci' => designerSuit,
            'Angelico' => designerSuit, 
            'Brioni' => designerSuit, 
            'Saint Laurent' => designerSuit,
            'Givenchy' => designerSuit, 
            'Prada' => designerSuit, 
            'Valentino' => designerSuit, 
            'Bottega Veneta' => designerSuit,
            'Hugo Boss' => designerSuit,
            'Dior' => designerSuit,
            '_' => panic!("Invalid suit title given.")
        match dressShirt {
            'Atu Body Couture' => dressShirt,
            'Prada' => dressShirt,
            'Isaac Sellam Experience' => dressShirt,
            'Thom Browne' => dressShirt, 
            'Eleventy' => dressShirt, 
            'Saint Laurent' => dressShirt,
            'Valentino' => dressShirt, 
            'Giorgio Armani' => dressShirt, 
            'Versace' => dressShirt, 
            'Salvatore Ferragamo' => dressShirt,
            'Philipp Plein' => dressShirt,
            'Brunello Cucinelli' => dressShirt,
            '_' => panic!("Invalid dress shirt title given.")
        }
        match dressPants {
            'Giuliva Heritage' => dressShirt,
            'Thom Sweeney' => dressShirt,
            'Umit Benan B+' => dressShirt,
            'Lemaire' => dressShirt, 
            'Tom Ford' => dressShirt, 
            'Saint Laurent' => dressShirt,
            'Valentino' => dressShirt, 
            'Giorgio Armani' => dressShirt, 
            'Moncler' => dressShirt, 
            'Ralph Lauren Purple Label' => dressShirt,
            'Barena Venezia' => dressShirt,
            'Jil Sander' => dressShirt
            '_' => panic!("Invalid dress pants title given.")
        }
        match dressTie {
            'Charvet' => dressTie,
            'Tom Ford' => dressTie,
            'Gucci' => dressTie,
            'Brunello Cucinelli' => dressTie, 
            'Tom Ford' => dressTie, 
            'Gucci' => dressTie, 
            '_' => panic!("Invalid dress tie title given.")
        }
        match topJacket {
            'Brioni' => topJacket,
            'Dolce & Gabbana' => topJacket, 
            'Ermenegildo Zegna' => topJacket, 
            'Prada' => topJacket,
            'Philipp Plein' => topJacket, 
            'Brunello Cucinelli' => topJacket, 
            'Givenchy' => topJacket, 
            'Thom Browne' => topJacket,
            'Maison Margiela' => topJacket,
            'Canali' => topJacket,
            'Etro' => topJacket,
            'Lemaire' => topJacket,
            'Tonello' => topJacket
            '_' => panic!("Invalid top jacket title given.")
        }
        }
        }
    fn madeRegular(self) -> str {
        match regularSuit {
            'Topman' => regularSuit,
            'Ted Baker London' => regularSuit
            'Peter Millar' => regularSuit 
            'River Island' => regularSuit 
            'Johnny Bigg' => regularSuit
            'Jack Victor' => regularSuit 
            'Suitsupply' => regularSuit 
            '_' => panic!("Invalid suit title given.")
        match dressShirt {
            'Theory' => dressShirt,
            'Officine Generale' => dressShirt
            'Emanuel Berg' => dressShirt 
            'Eton' => dressShirt 
            'Canali' => dressShirt
            'Reiss' => dressShirt 
            '_' => panic!("Invalid dress shirt title given.")
        }
        match dressPants {
            'Versace' => dressPants,
            'Loro Piana' => dressPants
            'Boglioli' => dressPants 
            'Golden Goose' => dressPants 
            'Samuelsohn' => dressPants
            'Brax' => dressPants 
            'Berle' => dressPants 
            '_' => panic!("Invalid dress pants title given.")
        }
        match dressTie {
            'Zegna' => dressTie,
            'Brioni' => dressTie
            'Ralph Lauren Purple Label' => dressTie 
            'Thom Browne' => dressTie 
            'Dries Van Noten' => dressTie
            'Nordstrom' => dressTie 
            'David Donahue' => dressTie 
            '_' => panic!("Invalid dress pants title given.")

        }
        match topJacket {
            'River Island' => topJacket,
            'Topman' => topJacket
            'ASOS Design' => topJacket 
            'Selected Homme' => topJacket 
            'Reiss' => topJacket
            '_' => panic!("Invalid dress pants title given.")
        }
        }
    


    fn output(complete_designer_suit: String, complete_regular_suit: String, result: String) -> String {
        format("{} {} {} = {}", complete_designer_suit, complete_regular_suit})
        format("{} = {}",suitSetting)
        println!("{:?}",output(complete_designer_suit, complete_regular_suit, result));
    }
}
