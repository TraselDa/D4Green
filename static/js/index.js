function toogle(y)
{

    const x = document.getElementById("data-details");
    if(x.style.display === "none")
    {
        document.getElementById("data-details").style.flex ="3";
        document.getElementById("data-details").style.display = "block";
        document.querySelector(".buttonVoir").innerHTML = "voir plus";
        document.getElementById(y.id).innerHTML = "voir moins";
        voir_details(y);
    } else
    {
        document.getElementById("data-details").style.flex ="0";
        document.getElementById("data-details").style.display = "none";
        document.querySelector(".buttonVoir").innerHTML = "voir plus";
        document.getElementById(y.id).innerHTML = "voir plus";
    }

    window.onclick = function (event){
        if(event.target === document.getElementById("data-details") && x.style.display === "block") {
            document.getElementById("data-details").style.flex ="0";
            document.getElementById("data-details").style.display = "none";
            document.querySelector(".buttonVoir").innerHTML = "voir plus";
            document.getElementById(y.id).innerHTML = "voir plus";
        }
    }

}

function voir_details(x)
{
    document.getElementById("titre").innerHTML = "details" + x.id;
}
