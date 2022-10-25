use std::io;

fn main() {
    println!("Which spell do you want the wizard to cast?");
    let mut spell = String::new();

    io::stdin()
        .read_line(&mut spell)
        .expect("You failed to cast a spell!");

    cast_spell(&spell);
}

fn cast_spell(quote: &str) {
    println!(
        "                                 {quote} 

                                .''''.''' 
                             .'   :
\\                          .:    :
 \\                        _:    :       ..----.._
  \\                    .:::.....:::.. .'         ''.
   \\                 .'  #-. .-######'     #        '.
    \\                 '.##'/ ' ################       :
     \\                  #####################         :
      \\               ..##.-.#### .''''###'.._        :
       \\             :--:########:            '.    .' :
        \\..__...--.. :--:#######.'   '.         '.     :
        :     :  : : '':'-:'':'::        .         '.  .'
        '---'''..: :    ':    '..'''.      '.        :'
           \\  :: : :     '      ''''''.     '.      .:
            \\ ::  : :     '            '.      '      :
             \\::   : :           ....' ..:       '     '.
              \\::  : :    .....#### .~~.:.             :
               \\':.:.:.:'#########.===. ~ |.'-.   . '''.. :
                \\    .'  ##########   _.' '. '-.       '''.
                :\\  :     ########          '.  '-.        :
               :  \\'    '   #### :           :.    '-.      :
              :  .'\\   :'  :     :             :      '-.    :
             : .'  .\\  '  :      :     :        :        '.   :
             ::   :  \\'  :.      :     :        :          '. :
             ::. :    \\  : :      :    ;        :           '.:
              : ':    '\\ :  :     :     :  :     :        ..'
                 :    ' \\ :        :     ;        :   .'''
                 '.   '  \\:                         :.''
                  .:..... \\:       :            ..''
                 '._____|'.\\......'''''''.:..'''
                            \\
                             -
        "
    );
}
