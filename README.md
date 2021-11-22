# test du connecteur EAS

<u>Pour contruire le programme de test</u>

Dans le fichier main.rs il faut mettre les bons credentials avant le build

<u>Puis taper la commande</u>

cargo build --bin rust-token --release

<u>Pour le lancer</u>

cargo run --bin eas_test path-du-fichier-pour-upload

<u>Pour lancer le programme de nettoyage</u>

cargo run --bin eas_delete Nom_du_ticket_a_supprimer