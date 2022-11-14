// mod plot;
// use plot::plot_gen;

mod mesh;
use mesh::Mesh;

mod constants;
use constants::{NX, NY};

fn main() {
    let mesh = Mesh::mesh_gen(5., 5.);
    println!("{:?}", mesh);
    println!("{}   {}", mesh.x[13], mesh.y[13]);
    // plot_gen();
}

// D=0.1; %diametro condotto in m
// Tw=100; %temperatura di parete in �C
// Ti=10; %temperatura del fluido all'ingresso in �C
// alfa=1.5*10^-7; %diffusivit� termica [m^2/s]
// lambda=0.6; %conduttivit� termica [W/m�C]
// let cp_h2o=4186; %calore specifico [J/kg�C]
// ro=1000; %densit� [kg/m^3]
// Qv=0.0001; %portata in volume [m^3/s]
// W=Qv/(pi*((D/2)^2));
// %discretizzazione
// nz=100;%input('numero nodi lungo l asse del condotto: ');
// nr=80;%input('numero nodi lungo il raggio: ');
// nodi=nz*nr; %numero totale di nodo
// %delta z e delta r
// delta_z=L/(nz-1); %spazio tra un nodo e l'altro lungo z
// delta_r=(D/2)/(nr-1); %spazio tra un nodo e l'altro lungo il raggio, problema asimmetrico
// %numero per colonne
// r(1,nodi)=zeros;
// z(1,nodi)=zeros;
// S(1,nodi)=zeros;
// for j=1:nz
//     for i=1:nr
//         k=(j-1)*nr+i;
//         z(k)=(j-1)*delta_z; %coordinate spaziali lungo z
//         r(k)=(i-1)*delta_r; %coordinate spaziali lungo r
//     end
// end

// %Profilo di velocit�
// u_z=2*W*(1-(r/(D/2)).^2);

// % Inizzializzo le matrici per comporre il mio sistema
// A(nodi,nodi)=zeros;
// b(nodi,1)=zeros;
// % Condizioni al contorno in z=0 - Temperatura in z=0 costante = 10�C
// for i=1:nr
//     A(i,i)=1;
//     b(i,1)=Ti;
// end
// % Condizioni al contorno in r=R - Temperatura costante a 100�C
// for i=nr:nr:nodi
//     A(i,i)=1;
//     b(i,1)=Tw;
// end
// % Condizoni al contorno in r=0 - Flusso di calore nullo in direzione radiale
// for i=nr+1:nr:nr*(nz-1)-1
//     A(i,i+1)=1;
//     A(i,i)=-1;
//     b(i,1)=0;
// end
// % Condizoni al contorno in z=L - dT/dz=0 - la temperatura rester� a profilo costante
// for i=nr*(nz-1)+1:nodi
//     A(i,i)=1;
//     A(i,i-nr)=-1;
//     b(i,1)=0;
// end
// % Modedllazione matematica dei nodi interni al dominio tramite le differenze finite - Equazione dell'energia
// for i=(nr+2):(2*nr)-1
//     for  j=i:nr:nr*(nz-1)
//         A(j,j+nr)=u_z(1,j)/2*delta_z;
//         A(j,j+1)=-alfa*(1/delta_r^2+1/(2*delta_r*r(j)));
//         A(j,j)=2*alfa/delta_r^2;
//         A(j,j-1)=-alfa*(1/delta_r^2-1/(2*delta_r*r(j)));
//         A(j,j-nr)=-u_z(1,j)/2*delta_z;
//         b(j,1)=0;
//     end
// end
