library(ncdf4)
ncin <- nc_open("u.nc")
u <- ncvar_get(ncin, "u")
for (i in 1:9) {
    print(quantile(u, type=i))
}
