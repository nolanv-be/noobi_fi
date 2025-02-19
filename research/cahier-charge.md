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
Quelques questions lui sont pose pour definir son profil, une fois identifier son niveau de connaissance on lui propose de creer son wallet avec les differentes options pour configurer les signataires. Un setup standard sera par exemple un server sur un raspberry pi avec un EOA qui permet de faciliter l'execution de transaction apres signature des deux autres signataire, un signataire est un passkey sur mobile via la version web noobi.fi, l'autre signataire est stocker sur pc dans la version applicatif, un signataire de recuperation est stocke dans son gestionnaire de mot de passe avec un cooldown d'une semaine.
Ce setup lui donne acces en read-only a distance 


---
# Profil
TODO

# Infrastructure
Noobi.fi est compose d'une interface web pouvant etre self-hosted ou non, d'un serveur self-hosted gerant les automates.

## Interface web
L'interface web permet d'acceder au dashboard, elle est static et generer entierement par un serveur trusted. La configuration de l'interface peut-etre partager entre les differentes versions.

### noobi.fi
La version publique de noobi.fi pourra etre utiliser comme version demo afin de commencer l'apprentissage et etre rediriger vers les autres versions. Cette version pourra egalement etre utiliser comme version read-only ne necessitant pas de VPN.

### Application
Une simple application cross-plateform permet d'avoir un webserver local et donne ainsi acces a une copie local de noobi.fi. Cette version est le meilleur compromis pour interagir rapidement avec noobi.fi. L'application ne permettra pas d'installer des modules ou de se passer de RPC mes permets d'enlever le risque de supply chain attack.

### Self-hosted
L'objectif a long terme est que chaque utilisateur est son propre mini serveur qui lui donne ainsi acces a toutes les fonctionnalites sans intermediaires. Cette version necessitera un VPN ou un moyen de passer les CGNAT & co pour y acceder a distance.

TODO

# Wallet
Un wallet est toujours un safe ou equivalent si non-evm, un wallet est compose de signataire et de signataire de recuperation. Les signataires servent a signer les transactions, l'execution peut etre potentiellement fait par un tiers, ils doivent etre difficile a extraire, facile a utiliser. Les signataire de recuperation quant a eux servent a recuperer l'acces au wallet en cas de perte du quorum des signataires, ils doivent difficile a perdre et seront donc plus a risque de fuiter.
TODO
