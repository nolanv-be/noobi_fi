# Introduction
noobi.fi est une application modulaire permettant de construire un dashboard personnalise pour suivre son portefeuille de crypto-actif.

L'application dispose d'une version web heberge par noobi.fi, cette version comporte quelques restrictions afin de proteger les utilisateurs. Une version local peut-etre lancer via une simple application cross-plateforme, celle-ci n'aura pas de restriction mais ne permettra pas de programmer des automates. Il sera egalement possible d'heberger un serveur qui debloquera l'ensemble des possibilite de noobi.fi.

noobi.fi fera toujours le pari que l'utilisateur n'est pas stupide, 5 minutes d'apprentissage est plus interessant que de brider son outil.

Un module central sera celui de l'apprentissage, NolanV accompagnera les utilisateurs dans la decouverte de la finance ainsi que la technologie sous-jacente de la finance decentralise.

# Utilisateur pro DeFi et developpeur
Le premier utilisateur est une personne ayant des connaissances en DeFi et etant egalement a l'aise en programmation, il est capable d'ecrire des lignes de commandes.

## Besoins
- Suivre son portefeuille d'actif
- Automatiser sa gestion
- Avoir le controle de son outil, afin de pouvoir le personnaliser
- Securiser son argent en reduisant les differents risques tel que la supply chain, blind signing, multisig etc...
- On-boarding ses proches via des dashboard simplifier pour leurs besoins specifique et limiter

## Arriver sur la plateforme




---
# On-board
Lorsqu'un utilisateur arrive sur noobi.fi pour la premiere fois, on definira ses besoins et le redirigera vers le contenu adapter a ses besoins.

On va prendre un exemple simple d'une personne ayant des connaissances basiques en finance et en DeFi, il utilise un wallet comme Metamask et utilise des applications simple comme Aave et Uniswap. Il souhaite approfondir ses connaissances, ameliorer et automatiser sa gestion patrimonial.

Une fois son profil cree sur noobi.fi, il sera redirige vers le dashboard d'apprentissage pre-configurer pour suivre les bases de la gestion de position, la securite de wallet etc.

Chaque cours aura un mise en pratique qui l'ameneront a setup son wallet, et configurer son safe avec plusieurs signers.

# Architecture applicatif
L'application est decompose en trois morceaux: une web application, un web serveur, des modules offrant des rpcs ou des module DeFi supplementaire.

## Web serveur
Le web serveur est le coeur du systeme, il sert de database pour les configurations du ou des utilisateurs, il permet aussi d'utiliser certains modules natifs comme les read/write vers des rpcs ou un wallet.

Le web serveur peut etre utilise par plusieurs utilisateurs, le proprietaire peut par restreindre ce qui permit sur son serveur, par exemple l'instance de noobi.fi laissera l'utilisateur configurer son dashboard, mais pas installer de nouveau module.

Le web serveur peut-etre installer via une simple application locallement afin d'avoir sa propre instance ou sur un serveur avec lxc active.
