function toogle(e){var{y:t,commune:n}=e;const o=document.getElementById("data-details");"none"===o.style.display?(o.style.flex="3",o.style.display="block",document.querySelector(".buttonVoir").innerHTML="voir plus",document.getElementById(t.id).innerHTML="voir moins",voir_details(n)):(o.style.flex="0",o.style.display="none",document.querySelector(".buttonVoir").innerHTML="voir plus",document.getElementById(t.id).innerHTML="voir plus"),window.onclick=function(e){e.target===document.getElementById("data-details")&&"block"===o.style.display&&(document.getElementById("data-details").style.flex="0",document.getElementById("data-details").style.display="none",document.querySelector(".buttonVoir").innerHTML="voir plus",document.getElementById(t.id).innerHTML="voir plus")}}function voir_details(e){document.getElementById("_POP_").innerHTML="Population : "+e.population,document.getElementById("_POP_").style.fontSize="20px",document.getElementById("_COM_").innerHTML=e.nom_com,document.getElementById("_COM_").style.fontWeight="Bold",document.getElementById("_COM_").style.fontSize="25px",document.getElementById("_NOM_R_DEP").innerHTML=e.nomdep+" & "+e.nomr,document.getElementById("_NOM_R_DEP").style.fontWeight="Bold",document.getElementById("_SCORE_R_DEP").innerHTML="REGION: "+e.score_global+" | DEPARTEMENT: "+e.score_global,document.getElementById("_SCORE_R_DEP").style.fontWeight="Bold",document.getElementById("_SCORE_R_DEP").style.color="Green",document.getElementById("_CLASSEMENT_").innerHTML="N : "+e.classement,document.getElementById("_CLASSEMENT_").style.fontWeight="Bold",document.getElementById("_CLASSEMENT_").style.color="Red",document.getElementById("_IRIS_._NC_").innerHTML="Iris: C: "+e.code_iris+" "+e.nom_iris,document.getElementById("_IRIS_._NC_").style.fontWeight="Bold",document.getElementById("_SCORE_G_").innerHTML="SCORE Global : "+e.score_global,document.getElementById("_SCORE_G_").style.fontWeight="Bold",document.getElementById("_GLOBAL_ACCES_").innerHTML="Global Acces : "+e.global_acces,document.getElementById("_GLOBAL_ACCES_").style.fontWeight="Bold",document.getElementById("_GLOBAL_COMP_").innerHTML="Global Competences : "+e.global_competences,document.getElementById("_GLOBAL_COMP_").style.fontWeight="Bold",document.getElementById("_ACCES_A_LINFO_").innerHTML="Acces a L information : "+e.acces_a_linformation,document.getElementById("_ACCES_A_LINFO_").style.fontWeight="Bold",document.getElementById("_ACCES_A_LINTERFACE_").innerHTML="Acces aux interfaces numeriques : "+e.acces_aux_interfaces_numerique,document.getElementById("_ACCES_A_LINTERFACE_").style.fontWeight="Bold",document.getElementById("_COMPETENCES_ADMIN_").innerHTML="Competences Administratives : "+e.competences_administratives,document.getElementById("_COMPETENCES_ADMIN_").style.fontWeight="Bold",document.getElementById("_COMPETENCES_NUM_").innerHTML="Competences numeriques : "+e.competences_numeriques,document.getElementById("_COMPETENCES_NUM_").style.fontWeight="Bold",document.getElementById("dwn_btn").href="/download/"+e.id}function loadDatatable(e=1,t,n,o,l){if(window.XMLHttpRequest){httpRequest=new XMLHttpRequest,httpRequest.onreadystatechange=function(){if(httpRequest.readyState===XMLHttpRequest.DONE)if(200===httpRequest.status){var e=JSON.parse(this.responseText);e.length>0&&fillTableau(e)}else alert("Il y a eu un problème avec la requête.")};var d={current_page:e};null!=t&&(d.last_id=parseInt(t)),null!=n&&(d.region=n),null!=o&&(d.departement=o),null!=l&&(d.commune=l),httpRequest.open("POST","/loadTable",!0),httpRequest.setRequestHeader("Content-Type","application/x-commune-paging;charset=UTF-8"),httpRequest.send(JSON.stringify(d))}}function fillTableau(e){var t=document.getElementById("table_body");t.innerHTML="";for(var n=0;n<e.length;n++){var o=e[n],l=o.id,d=t.insertRow();d.dataset.key=l,d.insertCell().innerText=o.nom_com,d.insertCell().innerText=o.nomdep,d.insertCell().innerText=o.nomr,d.insertCell().innerText=o.population,d.insertCell().innerText=o.score_global;var a=document.createElement("button");a.className="buttonVoir",a.innerText="Voir Plus",a.id="voir_"+l,a.addEventListener("click",toogle.bind(this,{y:a,commune:o})),d.insertCell().appendChild(a)}}function navigateNext(){var e=document.getElementById("table-items"),t=document.getElementById("table_body"),n=parseInt(e.dataset.index);n+=1,e.dataset.index=n;var o=t.rows.length;loadDatatable(n,t.rows[o-1].dataset.key)}function navigatePrev(){var e=parseInt(tables.dataset.index);e-=1;body.rows.length,body.rows[0];tables.dataset.index=e,loadDatatable(e,last_row.dataset.key)}function selectRegion(e){loadDatatable(1,void 0,document.getElementById("box-region").value)}function selectDepartement(e){var t=document.getElementById("box-departement").value,n=document.getElementById("box-region").value;loadDatatable(1,void 0,n="Tout"==n?void 0:n,t)}function selectCommune(e){var t=document.getElementById("box-region").value;t="Tout"==t?void 0:t;var n=document.getElementById("box-departement").value;loadDatatable(1,void 0,t,n="Tout"==n?void 0:n,document.getElementById("box-commune").value)}window.addEventListener("DOMContentLoaded",e=>{loadDatatable()});